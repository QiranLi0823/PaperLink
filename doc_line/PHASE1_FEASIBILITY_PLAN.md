# Paper Studio Phase 1 可行性计划

## 一、Phase 1 总目标

Roadmap 定位：

> 比 Typora 更懂论文的科研 Markdown 编辑器。类似 Typora + Overleaf。

Phase 1 的核心任务是将 Phase 0 的技术验证成果升级为一个可分发、可日常使用的桌面应用。

```
Phase 0（已完成）          Phase 1（本阶段）
     Web Demo          →      桌面应用
     JS 回退解析器      →      Rust WASM 引擎
     textarea 编辑      →      Monaco Editor
     无文件管理          →      本地工程管理
     无 BibTeX          →      文献引用系统
     HTML 预览          →      HTML + PDF 输出
```

---

## 二、Phase 0 已有资产盘点

| 资产 | 状态 | Phase 1 复用方式 |
|------|------|------------------|
| paper-core (Rust) | Parser + AST + Renderer 已完成 | 编译为 WASM，桌面应用直接调用 |
| grammar.pest | 完整语法定义 | 作为 Monaco Editor 语法高亮基础 |
| demo.pml | 完整示例论文 | 扩展为更丰富的测试用例 |
| JS 回退解析器 | 功能完整 | 作为 WASM 加载失败时的降级方案 |
| web/ 前端 | 双栏布局 + KaTeX | UI 设计参考，迁移到 React |

---

## 三、Phase 1 新增范围

### 3.1 必须完成（P0）

| 功能 | 说明 |
|------|------|
| Electron 桌面应用 | macOS + Windows，单窗口双栏布局 |
| Monaco Editor 集成 | 替代 textarea，PaperML 语法高亮 |
| Rust WASM 集成 | paper-core 编译为 WASM，做实时解析 |
| 文件系统集成 | 打开/保存 .pml 文件，管理工程目录 |
| HTML 导出 | 论文预览导出为独立 HTML 文件 |
| 错误诊断 | 语法错误行号提示，红色波浪线 |

### 3.2 应该完成（P1）

| 功能 | 说明 |
|------|------|
| BibTeX 管理 | 解析 .bib 文件，引用自动补全 |
| 引用格式化 | 根据引用风格模板格式化文献列表 |
| PDF 导出 | 通过 HTML → PDF（Puppeteer/Electron API）|
| 自动补全 | @section、@figure 等命令提示 |
| 工程模板 | 新建论文时选择空白模板 |

### 3.3 可以延后（P2）

| 功能 | 说明 |
|------|------|
| 图片拖拽插入 | 拖入图片自动生成 @figure 块 |
| 暗色模式 | 编辑器/预览双主题 |
| 自动保存 | 定时自动保存草稿 |
| 最近文件 | 启动页显示最近编辑的论文 |
| 多标签页 | 同时打开多篇论文 |

---

## 四、技术架构

### 4.1 架构总览

```
┌─────────────────────────────────────────────────────┐
│                  Electron Shell                       │
│  ┌─────────────────────────────────────────────────┐ │
│  │              React Application                    │ │
│  │  ┌───────────┐  ┌───────────┐  ┌─────────────┐ │ │
│  │  │  Sidebar   │  │  Editor   │  │   Preview    │ │ │
│  │  │  文件树     │  │  Monaco   │  │   HTML渲染    │ │ │
│  │  │  .bib列表   │  │  PaperML  │  │   KaTeX     │ │ │
│  │  └───────────┘  └───────────┘  └─────────────┘ │ │
│  └──────────────────────┬──────────────────────────┘ │
│                         │                             │
│  ┌──────────────────────▼──────────────────────────┐ │
│  │              paper-core (WASM)                    │ │
│  │   Parser  →  AST  →  Renderer  →  HTML           │ │
│  └─────────────────────────────────────────────────┘ │
│                         │                             │
│  ┌──────────────────────▼──────────────────────────┐ │
│  │           Node.js Main Process                    │ │
│  │   文件读写  │  BibTeX解析  │  PDF生成             │ │
│  └─────────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────┘
```

### 4.2 技术选型

| 层 | 技术 | 理由 |
|----|------|------|
| 桌面框架 | Electron 28+ | 成熟生态，Monaco 原生支持好 |
| UI 框架 | React 18 + TypeScript | 组件化开发，类型安全 |
| 编辑器 | Monaco Editor (VS Code 内核) | 语法高亮、自动补全、错误标记 |
| 核心引擎 | Rust → WASM (paper-core) | Phase 0 已有，直接复用 |
| 构建工具 | Vite + electron-vite | 快速 HMR，开箱即用 |
| 样式 | Tailwind CSS | 快速开发，易于维护 |
| BibTeX 解析 | citation-js (Node.js) | 成熟库，支持多种引用格式 |
| PDF 生成 | Puppeteer | HTML → PDF，支持分页和排版 |
| 包管理 | pnpm | 快速，节省磁盘空间 |

### 4.3 项目结构

```
paper-studio/
├── paper-core/                 # Rust 核心引擎（已有，小幅扩展）
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs
│       ├── ast.rs
│       ├── parser.rs
│       ├── grammar.pest
│       └── renderer.rs
│
├── desktop/                    # Electron 桌面应用（新建）
│   ├── package.json
│   ├── electron.vite.config.ts
│   ├── src/
│   │   ├── main/               # Electron 主进程
│   │   │   ├── index.ts        # 窗口管理
│   │   │   ├── file-system.ts  # 文件读写
│   │   │   ├── bibtex.ts       # BibTeX 解析
│   │   │   └── export.ts       # HTML/PDF 导出
│   │   ├── preload/            # 预加载脚本
│   │   │   └── index.ts        # IPC 桥接
│   │   └── renderer/           # React 渲染进程
│   │       ├── App.tsx
│   │       ├── components/
│   │       │   ├── Editor.tsx         # Monaco 编辑器
│   │       │   ├── Preview.tsx        # 论文预览
│   │       │   ├── Sidebar.tsx        # 文件树
│   │       │   ├── Toolbar.tsx        # 工具栏
│   │       │   └── StatusBar.tsx      # 状态栏
│   │       ├── hooks/
│   │       │   ├── usePaperEngine.ts  # WASM 引擎封装
│   │       │   └── useFileSystem.ts   # 文件操作封装
│   │       ├── store/
│   │       │   └── paperStore.ts      # Zustand 状态管理
│   │       └── styles/
│   │           └── index.css
│   └── resources/              # 应用图标等
│
├── web/                        # Web 前端（Phase 0，保留）
│   ├── index.html
│   ├── style.css
│   └── main.js
│
├── examples/                   # 示例论文
│   └── demo.pml
│
└── doc_line/                   # 文档
    ├── Paper_Studio_MVP_Roadmap.md
    ├── Paper_Studio_Phase0_Technical_Solution.md
    ├── PHASE0_DEVELOPMENT_PLAN.md
    └── PHASE1_FEASIBILITY_PLAN.md  ← 本文件
```

---

## 五、核心模块详细设计

### 5.1 Monaco Editor 集成

```
┌─────────────────────────────────────────┐
│  Monaco Editor (PaperML)                 │
│                                           │
│  @section Introduction                   │ ← 语法高亮
│  ~~                                      │ ← 错误波浪线
│  @figure{                                │
│    path = "│"  ← 自动补全                 │
│  }                                       │
│                                           │
│  @cite{vaswani2017│  ← 引用补全           │
│                                           │
└─────────────────────────────────────────┘
```

关键实现：

- **语法高亮**：基于 grammar.pest 定义 Monarch tokenizer
- **自动补全**：@section、@figure、@table、@equation 等命令提示
- **诊断功能**：调用 WASM parser 返回错误位置，Monaco setModelMarkers 显示
- **引用补全**：读取 .bib 文件，@cite{ 触发引用键补全

### 5.2 文件工程管理

```
Project/
├── paper.pml          ← 主文件
├── refs.bib           ← BibTeX 文献库
├── figures/           ← 图片目录
│   ├── framework.png
│   └── results.png
└── output/            ← 导出目录
    ├── paper.html
    └── paper.pdf
```

- 新建工程：创建上述目录结构和模板 .pml 文件
- 打开工程：读取 paper.pml + refs.bib，初始化编辑器
- 保存：Ctrl+S 写入 paper.pml
- 图片管理：支持拖入 figures/，自动生成 @figure 块

### 5.3 BibTeX 管理系统

```bibtex
@article{vaswani2017attention,
  title={Attention is all you need},
  author={Vaswani, Ashish and ...},
  journal={Advances in NIPS},
  year={2017}
}
```

功能：

- 解析 refs.bib，展示引用条目列表
- @cite{} 触发引用键自动补全
- 点击引用键跳转到文献详情
- 支持多种引用风格（IEEE、APA、Nature）

### 5.4 PDF 导出

```
PaperML → WASM Parser → Paper AST → HTML → Puppeteer → PDF
```

实现方案：

1. Rust Renderer 生成完整 HTML（含 CSS 打印样式）
2. 通过 IPC 将 HTML 发送到主进程
3. 主进程调用 Puppeteer 启动无头浏览器
4. 渲染 HTML 并生成 PDF
5. 保存到 output/ 目录

备选方案：Electron 内置 BrowserWindow 的 webContents.printToPDF()

### 5.5 WASM 集成方案

```typescript
// renderer/hooks/usePaperEngine.ts
import initWasm, { parse_and_render, parse_to_json } from 'paper-core';

export function usePaperEngine() {
  const [wasmReady, setWasmReady] = useState(false);

  useEffect(() => {
    initWasm().then(() => setWasmReady(true));
  }, []);

  const parse = useCallback((input: string) => {
    if (!wasmReady) {
      // 降级到 JS 回退解析器
      return jsFallbackParse(input);
    }
    try {
      return {
        html: parse_and_render(input),
        ast: JSON.parse(parse_to_json(input))
      };
    } catch (e) {
      return { error: e.message, positions: e.positions };
    }
  }, [wasmReady]);

  return { parse, wasmReady };
}
```

---

## 六、开发阶段划分

### 阶段 A：环境搭建（2-3 天）

| 任务 | 产出 |
|------|------|
| 初始化 Electron + React + Vite 项目 | 可启动的空白桌面窗口 |
| 配置 TypeScript、ESLint、Prettier | 统一的开发环境 |
| 搭建双栏布局（Shell + React Router） | 编辑器区 + 预览区 + 侧边栏骨架 |
| 集成 Monaco Editor | 可输入文本的编辑器 |

**验证**：双击应用图标，看到双栏界面，Monaco 可以输入 PaperML。

### 阶段 B：WASM 集成（3-4 天）

| 任务 | 产出 |
|------|------|
| paper-core 编译为 WASM | pkg/ 输出到 desktop/ 可引用 |
| wasm-bindgen 错误信息增强 | 返回行号、列号、错误类型 |
| React hook 封装 WASM 调用 | usePaperEngine |
| 实时解析 + 预览 | 编辑 PaperML 时右侧实时渲染 HTML |

**验证**：编辑 PaperML 文本，右侧预览 150ms 内更新，KaTeX 公式正确渲染。

### 阶段 C：编辑器增强（4-5 天）

| 任务 | 产出 |
|------|------|
| PaperML 语法高亮（Monarch） | @section 等关键字着色 |
| 自动补全提供者 | @figure{} 等代码片段 |
| 诊断功能（错误标记） | 语法错误行红色波浪线 + 信息 |
| @cite{} 引用补全 | 输入 @cite{ 弹出参考文献列表 |

**验证**：输入错误的 PaperML 即时看到红色标记，@cite{ 触发补全。

### 阶段 D：文件系统（3-4 天）

| 任务 | 产出 |
|------|------|
| 新建/打开/保存工程 | 完整文件操作流程 |
| 侧边栏文件树 | 显示工程目录结构 |
| 图片管理 | 拖入图片 → 复制到 figures/ |
| 最近文件列表 | 启动页最近工程入口 |

**验证**：新建工程 → 编辑 → 保存 → 关闭 → 重新打开，内容完整保留。

### 阶段 E：BibTeX 支持（3-4 天）

| 任务 | 产出 |
|------|------|
| .bib 文件解析（主进程） | 读取 refs.bib 生成引用列表 |
| 引用侧边栏 | 显示所有引用条目 |
| @cite{} 补全集成 | 补全列表来自 .bib 文件 |
| 参考文献列表自动生成 | 按引用顺序或字母序排列 |

**验证**：加载 .bib 文件后，@cite{} 可补全，预览中引用正确显示。

### 阶段 F：导出功能（3-4 天）

| 任务 | 产出 |
|------|------|
| HTML 导出 | 完整独立的 HTML 论文文件 |
| PDF 导出（Puppeteer） | 分页排版正确的 PDF |
| 导出配置对话框 | 选择输出格式和路径 |
| 打印样式 CSS | 适合学术论文排版的 @page/@media print |

**验证**：导出的 PDF 打开后排版正确，章节标题、图表、公式完整。

### 阶段 G：打磨发布（3-4 天）

| 任务 | 产出 |
|------|------|
| 应用打包（electron-builder） | macOS .dmg + Windows .exe |
| 自动更新 | electron-updater 集成 |
| 应用图标和启动画面 | 品牌化视觉 |
| 使用文档 | README + Quick Start 指南 |

**验证**：在干净的 macOS/Windows 上安装运行。

---

## 七、时间估算

| 阶段 | 内容 | 预计工期 |
|------|------|----------|
| A | 环境搭建 | 2-3 天 |
| B | WASM 集成 | 3-4 天 |
| C | 编辑器增强 | 4-5 天 |
| D | 文件系统 | 3-4 天 |
| E | BibTeX 支持 | 3-4 天 |
| F | 导出功能 | 3-4 天 |
| G | 打磨发布 | 3-4 天 |
| **总计** | | **21-28 天** |

约 **1 个月**，符合 Roadmap 中 Phase 1 的 3-6 个月预期（但 Roadmap 说的是从零开始；Phase 0 已完成核心引擎，大幅缩短了 Phase 1 工期）。

---

## 八、技术风险与对策

| 风险 | 影响 | 概率 | 对策 |
|------|------|------|------|
| wasm-pack 在 Electron 环境下兼容性问题 | 核心功能不可用 | 中 | 保留 JS 回退解析器作为降级方案 |
| Monaco Editor PaperML 语法高亮开发复杂 | 编辑体验差 | 低 | Monarch tokenizer 语法简单，有大量模板参考 |
| PDF 分页排版难以精确控制 | 导出质量差 | 中 | 优先保证 HTML 导出；PDF 允许分页位置存在小偏差 |
| BibTeX .bib 文件格式变体多 | 解析不完整 | 中 | citation-js 库覆盖主流格式；未知字段保留不丢弃 |
| 跨平台打包体积过大 | 下载体验差 | 中 | WASM + Electron 本体 ~150MB，属行业正常水平 |

---

## 九、成功标准

### 9.1 功能指标

| 功能 | 标准 |
|------|------|
| 编辑器 | Monaco Editor，PaperML 语法高亮，错误实时标记 |
| 解析 | WASM 引擎，1000 行文档 < 10ms |
| 预览 | 编辑即预览，150ms 内更新 |
| 文件管理 | 新建/打开/保存工程，支持相对路径图片 |
| 文献引用 | 加载 .bib 文件，@cite{} 补全，参考文献列表生成 |
| 导出 | HTML（100% 保真）、PDF（分页排版）|

### 9.2 性能指标

| 指标 | 目标 |
|------|------|
| 应用启动时间 | < 3 秒 |
| 解析 1000 行 | < 10ms（WASM）|
| 预览更新 | < 150ms（debounce） |
| PDF 导出 10 页 | < 5 秒 |
| 内存占用 | < 300MB（编辑中等规模论文）|

### 9.3 体验指标

- 科研人员可在 **5 分钟内** 从安装到写出第一篇 PaperML 论文
- 已熟悉 Markdown 的用户上手时间 **< 2 分钟**
- 编辑体验流畅，无明显卡顿或闪烁

---

## 十、Phase 1 不做的事

明确不纳入 Phase 1：

- Word/LaTeX 导出（Roadmap Phase 2/3）
- 模板系统 / 样式切换（Roadmap Phase 2）
- AI 辅助写作（Roadmap Phase 4）
- 多人协作 / 版本管理（Roadmap Phase 2）
- 云同步 / 账号系统
- 移动端支持
- 插件系统

---

## 十一、与 Phase 0 的关系

Phase 0 验证了核心链路：PaperML → AST → HTML。Phase 1 在此基础上做三件事：

1. **工程化**：从 Demo → 可分发应用
2. **编辑器专业化**：从 textarea → Monaco Editor（语法高亮 + 错误诊断 + 自动补全）
3. **写作工作流**：从单文件编辑 → 工程管理 + BibTeX + 导出

Phase 0 的 paper-core、grammar.pest、JS 回退解析器全部直接复用，核心引擎几乎不需要改动。

---

## 十二、推荐执行顺序

```
Phase 0（已完成）
    │
    ▼
Phase 1-A: 环境搭建        ← 脚手架，先让 Electron 跑起来
    │
    ▼
Phase 1-B: WASM 集成       ← 核心链路，验证桌面端也能用 WASM
    │
    ▼
Phase 1-C: 编辑器增强       ← 开发体验，语法高亮和错误提示
    │
    ▼
Phase 1-D: 文件系统         ← 用户闭环，能打开和保存文件
    │
    ▼
Phase 1-E: BibTeX 支持      ← 学术刚需，文献引用是论文写作核心
    │
    ▼
Phase 1-F: 导出功能         ← 产出物，HTML 和 PDF
    │
    ▼
Phase 1-G: 打磨发布         ← 打包、文档、发布
```

每完成一个阶段即可获得一个可运行的中间产物，降低集成风险。

---

*文档版本：v0.1*
*创建日期：2026-07-30*
*关联文档：Paper_Studio_MVP_Roadmap.md、Paper_Studio_Phase0_Technical_Solution.md、PHASE0_DEVELOPMENT_PLAN.md*
