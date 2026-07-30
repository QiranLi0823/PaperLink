# Paper Studio Phase 0 开发计划

## 项目概述

**目标**：实现 PaperML → AST → HTML 的完整链路，交付一个可运行的 Web Demo

**技术栈**：Rust + WASM + CodeMirror 6

**预计工期**：分 9 个步骤完成

---

## 开发任务列表

### Step 1: 项目初始化
- [x] 创建 Rust workspace 结构
- [x] 配置 Cargo.toml（依赖：pest, serde, wasm-bindgen）
- [x] 创建基础目录结构
- [x] 验证：`cargo check` 通过

### Step 2: 定义 AST 节点
- [x] 定义 Document 结构
- [x] 定义 Block 枚举（Section, Paragraph, Figure, Table, Equation）
- [x] 定义 Inline 枚举（Text, Citation, Reference, Math）
- [x] 添加 serde 序列化支持
- [x] 验证：AST 可以序列化为 JSON

### Step 3: 实现 PaperML 语法（pest grammar）
- [x] 定义基础语法规则（document, block, paragraph）
- [x] 定义 section 语法
- [x] 定义 figure 语法
- [x] 定义 table 语法
- [x] 定义 equation 语法
- [x] 定义 citation 和 reference 语法
- [x] 验证：grammar.pest 语法正确，pest 可以编译

### Step 4: 实现 Parser
- [x] 实现 pest Parser 结构
- [x] 实现 parse_document 函数
- [x] 实现各 block 类型的解析
- [x] 实现 inline 元素的解析
- [x] 错误处理
- [x] 验证：单元测试通过（6/6 passed）

### Step 5: 实现 HTML Renderer
- [x] 实现 render_document 函数
- [x] 实现 section 渲染
- [x] 实现 paragraph 渲染（含 inline 元素）
- [x] 实现 figure 渲染
- [x] 实现 table 渲染
- [x] 实现 equation 渲染（KaTeX 格式）
- [x] 验证：AST 正确转换为 HTML

### Step 6: WASM 绑定
- [x] 配置 wasm-bindgen
- [x] 导出 parse_and_render 函数
- [x] 导出 parse_to_json 函数
- [x] 错误处理（返回 JS 友好的错误）
- [x] 验证：WASM 绑定代码完成，需本地构建

### Step 7: Web 前端 - 基础界面
- [x] 创建 index.html（双栏布局）
- [x] 创建 style.css（编辑器 + 预览样式）
- [x] 使用 textarea 作为编辑器（Phase 0 简化版）
- [x] 验证：页面可以正常显示

### Step 8: Web 前端 - 功能集成
- [x] 实现 JS 回退解析器
- [x] 实现编辑器内容变化监听
- [x] 实现实时渲染
- [x] 集成 KaTeX（数学公式渲染）
- [x] 错误提示
- [x] 验证：编辑 PaperML 时右侧实时预览

### Step 9: 端到端验证
- [x] 创建完整示例论文 demo.pml
- [x] 测试所有语法元素
- [x] JS 回退解析器性能良好
- [x] 验证：完整论文可以正确渲染

---

## 当前进度

| Step | 状态 | 完成时间 |
|------|------|----------|
| Step 1: 项目初始化 | ✅ 完成 | 2026-07-30 |
| Step 2: AST 节点 | ✅ 完成 | 2026-07-30 |
| Step 3: pest 语法 | ✅ 完成 | 2026-07-30 |
| Step 4: Parser | ✅ 完成 | 2026-07-30 |
| Step 5: HTML Renderer | ✅ 完成 | 2026-07-30 |
| Step 6: WASM 绑定 | ✅ 完成 | 2026-07-30 |
| Step 7: Web 基础界面 | ✅ 完成 | 2026-07-30 |
| Step 8: 功能集成 | ✅ 完成 | 2026-07-30 |
| Step 9: 端到端验证 | ✅ 完成 | 2026-07-30 |

**注意**：WASM 绑定代码已完成，但 wasm-bindgen 工具需要在本地环境编译安装。
Web 前端包含完整的 JS 回退解析器，可在没有 WASM 的情况下正常工作。

---

## 验证检查点

### Parser 验证用例

```paper
@section Introduction

This is a test paragraph with @cite{ref2024} citation.

@figure{
  path = "test.png"
  caption = "Test figure"
}
```

预期 AST 输出：
```json
{
  "content": [
    { "type": "section", "level": 1, "title": "Introduction", "children": [...] },
    { "type": "paragraph", "content": [...] },
    { "type": "figure", "path": "test.png", "caption": "Test figure" }
  ]
}
```

### 性能基准

| 指标 | 目标值 |
|------|--------|
| 解析 100 行 | < 1ms |
| 解析 1000 行 | < 10ms |
| WASM 包大小 | < 500KB (gzip) |
| 首次渲染 | < 100ms |

---

## 目录结构

```
paper-studio/
├── Cargo.toml                  # Workspace 配置
├── paper-core/                 # Rust 核心引擎
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs              # WASM 入口
│       ├── ast.rs              # AST 定义
│       ├── parser.rs           # Parser 实现
│       ├── grammar.pest        # PaperML 语法
│       └── renderer.rs         # HTML 渲染器
├── web/                        # Web 前端
│   ├── index.html
│   ├── style.css
│   ├── main.js
│   └── pkg/                    # WASM 编译输出
├── examples/                   # 示例文件
│   └── demo.pml
└── PHASE0_DEVELOPMENT_PLAN.md  # 本文件
```
