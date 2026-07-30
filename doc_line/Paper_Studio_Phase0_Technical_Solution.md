# Paper Studio Phase 0 技术验证方案

## 一、Phase 0 总目标

Phase 0 的目标不是开发完整的软件，而是验证 Paper Studio 最核心的技术路线：

> 能否定义一种比 LaTeX 更友好、比 Markdown 更专业、又能被机器理解的论文描述语言。

核心验证链路：

```
用户输入 PaperML  →  Parser 解析  →  Paper AST  →  HTML Renderer  →  浏览器预览
```

最终目标：

输入：

```paper
@section Introduction

This paper proposes a new trajectory prediction framework.

@figure{
    path="framework.png"
    caption="Overview of the proposed framework."
}
```

输出：

一个网页形式的论文实时预览（左侧编辑器 + 右侧预览）。

------------------------------------------------------------------------

## 二、Phase 0 不做什么

为了保证研发效率，Phase 0 不考虑：

- Word 导出
- LaTeX 导出
- AI 写论文
- 用户系统
- 云同步
- 多人协作

这些属于后续产品阶段。Phase 0 只验证：

> 论文是否可以被一种新的结构化语言表示。

------------------------------------------------------------------------

## 三、Phase 0 技术架构

### 3.1 架构总览

```
┌─────────────────────────────────────────────────────────┐
│                    Web Application                       │
│  ┌─────────────────┐        ┌─────────────────────────┐ │
│  │   Editor (左)    │        │      Preview (右)       │ │
│  │   CodeMirror 6   │   →    │      HTML 渲染结果       │ │
│  └─────────────────┘        └─────────────────────────┘ │
└───────────────────────────────┬─────────────────────────┘
                                │
                    ┌───────────▼───────────┐
                    │   paper-core (WASM)   │
                    │  ┌─────────────────┐  │
                    │  │  Lexer/Parser   │  │
                    │  │     (pest)      │  │
                    │  └────────┬────────┘  │
                    │  ┌────────▼────────┐  │
                    │  │   Paper AST     │  │
                    │  └────────┬────────┘  │
                    │  ┌────────▼────────┐  │
                    │  │  HTML Renderer  │  │
                    │  └─────────────────┘  │
                    └───────────────────────┘
```

### 3.2 技术选型

| 模块 | 技术 | 理由 |
|------|------|------|
| Parser | Rust + pest | 声明式 PEG 语法，易于维护，性能优秀 |
| AST | Rust structs + serde | 类型安全，可序列化为 JSON |
| Renderer | Rust → HTML String | 直接生成 HTML，简单高效 |
| WASM 绑定 | wasm-bindgen | Rust 编译成 WASM 的标准方案 |
| Web 前端 | Vanilla JS + CodeMirror 6 | 轻量级，Phase 0 不需要框架 |
| 构建工具 | wasm-pack | 一键编译 + 生成 npm 包 |

### 3.3 为什么选择 Rust + WASM

1. **性能**：Rust 编译后性能接近原生，远超 Python/JavaScript
2. **复用性**：核心引擎一次编写，可同时用于 Web（WASM）和桌面（Native）
3. **类型安全**：AST 结构在编译期保证正确性
4. **后续演进**：Phase 1 的 Electron 应用可直接复用 WASM 模块

### 3.4 项目结构

```
paper-studio/
├── paper-core/                 # Rust 核心引擎
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs              # WASM 入口
│       ├── parser/
│       │   ├── mod.rs
│       │   ├── grammar.pest    # PaperML 语法定义
│       │   └── lexer.rs
│       ├── ast/
│       │   ├── mod.rs
│       │   └── nodes.rs        # AST 节点定义
│       └── renderer/
│           ├── mod.rs
│           └── html.rs         # HTML 渲染器
│
├── web/                        # Web 前端
│   ├── index.html
│   ├── style.css
│   └── main.js
│
├── examples/                   # 示例论文
│   └── demo.pml
│
└── README.md
```

### 3.5 核心模块职责

| 模块 | 职责 |
|------|------|
| paper-core | Parser + AST + Renderer，编译为 WASM |
| web | 前端界面，调用 WASM 模块实现实时预览 |
| examples | 测试用的 PaperML 示例文件 |

------------------------------------------------------------------------

## 四、PaperML 语言设计

PaperML 类似 Markdown，但面向科学论文设计。

**对比示例**：

| 场景 | Markdown | PaperML |
|------|----------|---------|
| 章节 | `# Introduction` | `@section Introduction` |
| 图片 | `![](img.png)` | `@figure{ path="img.png" caption="..." }` |
| 公式 | `$y=f(x)$` | `@equation{ y=f(x) }` |
| 引用 | 无原生支持 | `@cite{key}` |

### 4.1 语法元素定义

#### 基础文本（Paragraph）

```paper
This paper proposes a novel framework.
```

AST：

```json
{
  "type": "paragraph",
  "content": "This paper proposes a novel framework."
}
```

#### 章节（Section）

```paper
@section Introduction
@subsection Background
@subsubsection Problem Definition
```

AST：

```json
{
  "type": "section",
  "level": 1,
  "title": "Introduction",
  "children": []
}
```

#### 图片（Figure）

```paper
@figure{
  path = "framework.png"
  caption = "Overview of the proposed framework."
  label = "fig:framework"
}
```

AST：

```json
{
  "type": "figure",
  "path": "framework.png",
  "caption": "Overview of the proposed framework.",
  "label": "fig:framework"
}
```

#### 表格（Table）

```paper
@table{
  caption = "Performance Comparison"
  label = "tab:results"
  columns = ["Method", "ADE", "FDE"]
  rows = [
    ["Ours", "0.72", "1.54"],
    ["Baseline", "0.89", "1.82"]
  ]
}
```

AST：

```json
{
  "type": "table",
  "caption": "Performance Comparison",
  "label": "tab:results",
  "columns": ["Method", "ADE", "FDE"],
  "rows": [["Ours", "0.72", "1.54"], ["Baseline", "0.89", "1.82"]]
}
```

#### 公式（Equation）

```paper
@equation{
  content = "E = mc^2"
  label = "eq:einstein"
}
```

AST：

```json
{
  "type": "equation",
  "content": "E = mc^2",
  "label": "eq:einstein"
}
```

#### 引用（Citation）

```paper
According to @cite{vaswani2017attention}, the Transformer architecture...
```

AST：

```json
{
  "type": "citation",
  "key": "vaswani2017attention"
}
```

#### 交叉引用（Reference）

```paper
As shown in @ref{fig:framework} and @ref{tab:results}...
```

AST：

```json
{
  "type": "reference",
  "target": "fig:framework"
}
```

### 4.2 pest 语法定义（grammar.pest）

```pest
// 文档结构
document = { SOI ~ (block)* ~ EOI }
block = { section | figure | table | equation | paragraph }

// 章节
section = { "@section" ~ title }
subsection = { "@subsection" ~ title }
title = { (!NEWLINE ~ ANY)+ }

// 图片
figure = { "@figure" ~ "{" ~ figure_attrs ~ "}" }
figure_attrs = { (figure_attr ~ NEWLINE*)* }
figure_attr = { ("path" | "caption" | "label") ~ "=" ~ quoted_string }

// 表格
table = { "@table" ~ "{" ~ table_attrs ~ "}" }

// 公式
equation = { "@equation" ~ "{" ~ equation_content ~ "}" }

// 引用
citation = { "@cite" ~ "{" ~ cite_key ~ "}" }
cite_key = { (ASCII_ALPHANUMERIC | "_")+ }

// 交叉引用
reference = { "@ref" ~ "{" ~ ref_target ~ "}" }

// 段落
paragraph = { (!(block_start) ~ ANY)+ }
block_start = { "@section" | "@subsection" | "@figure" | "@table" | "@equation" }

// 基础类型
quoted_string = { "\"" ~ (!"\"" ~ ANY)* ~ "\"" }
WHITESPACE = _{ " " | "\t" }
```

------------------------------------------------------------------------

## 五、Parser 实现

### 5.1 Parser 职责

将 PaperML 文本转换为结构化的 Paper AST：

```
PaperML 字符串  →  Lexer  →  Token 流  →  Parser  →  AST
```

### 5.2 Rust 实现结构

```rust
// src/parser/mod.rs
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "parser/grammar.pest"]
pub struct PaperMLParser;

pub fn parse(input: &str) -> Result<Document, ParseError> {
    let pairs = PaperMLParser::parse(Rule::document, input)?;
    build_ast(pairs)
}
```

### 5.3 核心依赖

```toml
[dependencies]
pest = "2.7"
pest_derive = "2.7"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
wasm-bindgen = "0.2"
```

------------------------------------------------------------------------

## 六、AST 设计

### 6.1 设计原则

AST 是整个系统的核心，设计为 **Scientific Document Object Model**（类似 HTML DOM）。

**不要**设计成 LaTeX AST，而应该是：

- 语义化的论文结构
- 与渲染目标无关
- 便于序列化和操作

### 6.2 论文结构树

```
Paper
├── Meta (title, authors, abstract)
├── Section[]
│   ├── Title
│   ├── Paragraph[]
│   ├── Figure[]
│   ├── Table[]
│   ├── Equation[]
│   └── Section[] (嵌套子章节)
└── References[]
```

### 6.3 Rust 类型定义

```rust
// src/ast/nodes.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Document {
    pub meta: Option<Meta>,
    pub content: Vec<Block>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Meta {
    pub title: Option<String>,
    pub authors: Vec<String>,
    pub abstract_text: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Block {
    Section(Section),
    Paragraph(Paragraph),
    Figure(Figure),
    Table(Table),
    Equation(Equation),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Section {
    pub level: u8,
    pub title: String,
    pub label: Option<String>,
    pub children: Vec<Block>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Paragraph {
    pub content: Vec<Inline>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Inline {
    Text(String),
    Citation { key: String },
    Reference { target: String },
    Math { content: String },
    Bold(Vec<Inline>),
    Italic(Vec<Inline>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Figure {
    pub path: String,
    pub caption: Option<String>,
    pub label: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Table {
    pub caption: Option<String>,
    pub label: Option<String>,
    pub columns: Vec<String>,
    pub rows: Vec<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Equation {
    pub content: String,
    pub label: Option<String>,
}
```

------------------------------------------------------------------------

## 七、HTML Renderer

### 7.1 渲染流程

```
AST  →  HTML Renderer  →  HTML String  →  浏览器渲染
```

### 7.2 Rust 实现

```rust
// src/renderer/html.rs
use crate::ast::*;

pub fn render(doc: &Document) -> String {
    let mut html = String::new();
    html.push_str("<article class=\"paper\">\n");
    
    for block in &doc.content {
        html.push_str(&render_block(block));
    }
    
    html.push_str("</article>\n");
    html
}

fn render_block(block: &Block) -> String {
    match block {
        Block::Section(s) => render_section(s),
        Block::Paragraph(p) => render_paragraph(p),
        Block::Figure(f) => render_figure(f),
        Block::Table(t) => render_table(t),
        Block::Equation(e) => render_equation(e),
    }
}

fn render_section(section: &Section) -> String {
    let tag = format!("h{}", section.level.min(6));
    let id = section.label.as_deref().unwrap_or("");
    let mut html = format!("<{} id=\"{}\">{}</{}>\n", tag, id, section.title, tag);
    
    for child in &section.children {
        html.push_str(&render_block(child));
    }
    html
}

fn render_figure(fig: &Figure) -> String {
    let id = fig.label.as_deref().unwrap_or("");
    let caption = fig.caption.as_deref().unwrap_or("");
    format!(
        "<figure id=\"{}\">\n  <img src=\"{}\" alt=\"{}\">\n  <figcaption>{}</figcaption>\n</figure>\n",
        id, fig.path, caption, caption
    )
}

fn render_equation(eq: &Equation) -> String {
    let id = eq.label.as_deref().unwrap_or("");
    format!(
        "<div class=\"equation\" id=\"{}\">\n  \\[{}\\]\n</div>\n",
        id, eq.content
    )
}
```

### 7.3 WASM 导出接口

```rust
// src/lib.rs
use wasm_bindgen::prelude::*;

mod ast;
mod parser;
mod renderer;

#[wasm_bindgen]
pub fn parse_and_render(input: &str) -> Result<String, JsValue> {
    let doc = parser::parse(input)
        .map_err(|e| JsValue::from_str(&e.to_string()))?;
    Ok(renderer::html::render(&doc))
}

#[wasm_bindgen]
pub fn parse_to_json(input: &str) -> Result<String, JsValue> {
    let doc = parser::parse(input)
        .map_err(|e| JsValue::from_str(&e.to_string()))?;
    serde_json::to_string_pretty(&doc)
        .map_err(|e| JsValue::from_str(&e.to_string()))
}
```

------------------------------------------------------------------------

## 八、Web 前端实现

### 8.1 技术选型

- **编辑器**：CodeMirror 6（轻量、可扩展、支持自定义语法高亮）
- **框架**：Vanilla JS（Phase 0 不需要 React/Vue）
- **样式**：原生 CSS（简洁的双栏布局）

### 8.2 页面结构

```html
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <title>Paper Studio - Phase 0</title>
    <link rel="stylesheet" href="style.css">
    <!-- CodeMirror -->
    <link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/@codemirror/...">
    <!-- KaTeX for math rendering -->
    <link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/katex@0.16/dist/katex.min.css">
</head>
<body>
    <div class="container">
        <div class="editor-pane">
            <div class="toolbar">
                <span class="title">PaperML Editor</span>
            </div>
            <div id="editor"></div>
        </div>
        <div class="preview-pane">
            <div class="toolbar">
                <span class="title">Preview</span>
            </div>
            <div id="preview"></div>
        </div>
    </div>
    
    <script type="module" src="main.js"></script>
</body>
</html>
```

### 8.3 核心 JavaScript

```javascript
// main.js
import init, { parse_and_render } from './pkg/paper_core.js';

let editor;
let previewEl;

async function main() {
    // 初始化 WASM
    await init();
    
    previewEl = document.getElementById('preview');
    
    // 初始化 CodeMirror 编辑器
    editor = createEditor(document.getElementById('editor'));
    
    // 监听编辑器变化，实时渲染
    editor.on('change', debounce(updatePreview, 150));
    
    // 初始渲染
    updatePreview();
}

function updatePreview() {
    const content = editor.getValue();
    try {
        const html = parse_and_render(content);
        previewEl.innerHTML = html;
        // 渲染数学公式
        renderMath(previewEl);
    } catch (e) {
        previewEl.innerHTML = `<div class="error">${e}</div>`;
    }
}

function debounce(fn, delay) {
    let timer;
    return function(...args) {
        clearTimeout(timer);
        timer = setTimeout(() => fn.apply(this, args), delay);
    };
}

main();
```

### 8.4 样式设计

```css
/* style.css */
* {
    margin: 0;
    padding: 0;
    box-sizing: border-box;
}

body {
    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif;
    height: 100vh;
    overflow: hidden;
}

.container {
    display: flex;
    height: 100%;
}

.editor-pane, .preview-pane {
    flex: 1;
    display: flex;
    flex-direction: column;
    border-right: 1px solid #e0e0e0;
}

.toolbar {
    padding: 12px 16px;
    background: #f5f5f5;
    border-bottom: 1px solid #e0e0e0;
    font-weight: 600;
}

#editor, #preview {
    flex: 1;
    overflow: auto;
    padding: 16px;
}

/* 论文预览样式 */
.paper {
    max-width: 800px;
    margin: 0 auto;
    font-family: 'Times New Roman', serif;
    line-height: 1.6;
}

.paper h1 { font-size: 1.5em; margin: 1em 0 0.5em; }
.paper h2 { font-size: 1.3em; margin: 1em 0 0.5em; }
.paper h3 { font-size: 1.1em; margin: 1em 0 0.5em; }

.paper figure {
    margin: 1.5em 0;
    text-align: center;
}

.paper figure img {
    max-width: 100%;
}

.paper figcaption {
    margin-top: 0.5em;
    font-size: 0.9em;
    color: #666;
}

.paper .equation {
    margin: 1em 0;
    text-align: center;
}

.paper table {
    width: 100%;
    border-collapse: collapse;
    margin: 1em 0;
}

.paper th, .paper td {
    border: 1px solid #ddd;
    padding: 8px;
    text-align: left;
}

.error {
    color: #d32f2f;
    padding: 1em;
    background: #ffebee;
    border-radius: 4px;
}
```

------------------------------------------------------------------------

## 九、构建与部署

### 9.1 开发环境要求

- Rust 1.70+
- wasm-pack
- Node.js 18+ (用于本地开发服务器)

### 9.2 构建命令

```bash
# 安装 wasm-pack
cargo install wasm-pack

# 构建 WASM 模块
cd paper-core
wasm-pack build --target web --out-dir ../web/pkg

# 启动本地服务器
cd ../web
npx serve .
```

### 9.3 CI/CD 配置（GitHub Actions）

```yaml
name: Build and Deploy

on:
  push:
    branches: [main]

jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      
      - name: Install Rust
        uses: dtolnay/rust-toolchain@stable
        
      - name: Install wasm-pack
        run: cargo install wasm-pack
        
      - name: Build WASM
        run: |
          cd paper-core
          wasm-pack build --target web --out-dir ../web/pkg
          
      - name: Deploy to GitHub Pages
        uses: peaceiris/actions-gh-pages@v3
        with:
          github_token: ${{ secrets.GITHUB_TOKEN }}
          publish_dir: ./web
```

------------------------------------------------------------------------

## 十、验证案例

测试不能使用简单 Demo，应该选择真实论文进行验证。

### 10.1 测试用例：IEEE TITS 论文

**目标论文结构**：

```
Introduction
Related Work
  ├── Trajectory Prediction
  └── Transformer in Motion Forecasting
Method
  ├── Problem Formulation
  ├── Network Architecture
  └── Training Strategy
Experiments
  ├── Datasets
  ├── Metrics
  ├── Comparison with State-of-the-art
  └── Ablation Study
Conclusion
References
```

**包含元素**：

- 多级 Section
- Figure（框架图、可视化结果）
- Table（性能对比表、消融实验表）
- Equation（损失函数、公式推导）
- Citation（文献引用）

### 10.2 示例 PaperML 文件

```paper
@meta{
  title = "A Novel Trajectory Prediction Framework"
  authors = ["Author One", "Author Two"]
}

@abstract{
  Trajectory prediction is essential for autonomous driving...
}

@section Introduction

Predicting the future trajectories of surrounding agents is crucial
for autonomous vehicles @cite{gupta2018social}. As shown in 
@ref{fig:overview}, our framework consists of three main components.

@figure{
  path = "figures/overview.png"
  caption = "Overview of the proposed framework."
  label = "fig:overview"
}

@section Method

@subsection Problem Formulation

Given the observed trajectory @equation{ X = \{x_1, ..., x_T\} },
we aim to predict the future trajectory @equation{ Y = \{y_{T+1}, ..., y_{T+H}\} }.

@subsection Loss Function

The total loss is defined as:

@equation{
  content = "\\mathcal{L} = \\mathcal{L}_{pred} + \\lambda \\mathcal{L}_{reg}"
  label = "eq:loss"
}

@section Experiments

@subsection Quantitative Results

@table{
  caption = "Comparison with state-of-the-art methods on ETH-UCY dataset"
  label = "tab:comparison"
  columns = ["Method", "ADE", "FDE"]
  rows = [
    ["Social-GAN", "0.81", "1.52"],
    ["Trajectron++", "0.67", "1.18"],
    ["Ours", "0.58", "1.02"]
  ]
}

As shown in @ref{tab:comparison}, our method outperforms...

@section Conclusion

In this paper, we proposed a novel trajectory prediction framework...
```

------------------------------------------------------------------------

## 十一、Phase 0 交付物

经过 1-2 个月开发，应完成：

| 交付物 | 说明 |
|--------|------|
| **PaperML 规范 v0.1** | 支持 Section, Paragraph, Figure, Table, Equation, Citation, Reference |
| **paper-core** | Rust 核心引擎，包含 Parser + AST + HTML Renderer |
| **WASM 模块** | paper-core 编译产物，可在浏览器中运行 |
| **Web Demo** | 左右分栏界面，左侧编辑 PaperML，右侧实时预览 |
| **示例论文** | 完整的测试用例 demo.pml |

------------------------------------------------------------------------

## 十二、Phase 0 成功标准

### 指标 1：表达能力

| 元素 | 支持 | 说明 |
|------|------|------|
| Section | ✓ | 支持多级章节 |
| Paragraph | ✓ | 支持行内元素 |
| Figure | ✓ | 支持 caption 和 label |
| Table | ✓ | 支持结构化表格数据 |
| Equation | ✓ | 支持 LaTeX 公式语法 |
| Citation | ✓ | 支持文献引用 |
| Reference | ✓ | 支持交叉引用 |

### 指标 2：性能指标

| 指标 | 目标 |
|------|------|
| 解析速度 | < 10ms（1000 行文档）|
| WASM 包大小 | < 500KB（gzip 后）|
| 首次渲染 | < 100ms |
| 增量渲染 | < 50ms |

### 指标 3：用户体验

- 科研人员愿意使用 PaperML 替代直接写 LaTeX
- 语法易于学习，5 分钟内上手
- 实时预览流畅，无明显卡顿

------------------------------------------------------------------------

## 十三、Phase 0 后续演进

完成 Phase 0 后的技术路线：

```
                    PaperML
                       │
                       ▼
                  Paper AST
                       │
         ┌─────────────┼─────────────┐
         ▼             ▼             ▼
       HTML          LaTeX         DOCX
     (Phase 0)     (Phase 2)    (Phase 3)
         │
         ▼
       PDF
```

**Phase 1 扩展计划**：

- Electron 桌面应用封装
- 语法高亮（CodeMirror 扩展）
- 错误提示和自动补全
- 本地文件系统集成
- BibTeX 支持

------------------------------------------------------------------------

## 总结

Phase 0 的本质：

> 先建立"论文的结构化表示标准"，再开发论文编辑器。

技术路线类比：

| 领域 | 标准 | 工具 | 生态 |
|------|------|------|------|
| Web | HTML | 浏览器 | Web 生态 |
| 科研写作 | **PaperML** | **Paper Studio** | 科研写作生态 |

**Rust + WASM 的优势**：

1. 核心引擎一次编写，多端复用
2. 性能接近原生，用户体验流畅
3. 类型安全，减少运行时错误
4. 为 Phase 1 的 Electron 应用打好基础
