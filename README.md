# Paper Studio - Phase 0

一个面向科研论文的写作平台技术验证项目。

## 项目概述

Paper Studio 通过 PaperML（科研论文标记语言）作为核心中间表示，实现论文的实时预览和多格式导出。

**Phase 0 目标**：验证 PaperML → AST → HTML 的核心技术链路。

## 快速开始

### 方法 1：使用 JS 回退解析器（推荐，无需构建）

```bash
cd web
python3 -m http.server 8080
# 打开浏览器访问 http://localhost:8080
```

### 方法 2：构建 WASM 模块（需要 Rust 环境）

```bash
# 安装依赖
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

# 构建
./build.sh

# 运行
cd web && python3 -m http.server 8080
```

## 项目结构

```
paper-studio/
├── paper-core/           # Rust 核心引擎
│   ├── src/
│   │   ├── lib.rs        # WASM 入口
│   │   ├── ast.rs        # AST 节点定义
│   │   ├── parser.rs     # PaperML 解析器
│   │   ├── grammar.pest  # PaperML 语法定义
│   │   └── renderer.rs   # HTML 渲染器
│   └── Cargo.toml
├── web/                  # Web 前端
│   ├── index.html
│   ├── style.css
│   ├── main.js           # 包含 JS 回退解析器
│   └── pkg/              # WASM 编译输出
├── examples/             # 示例文件
│   └── demo.pml
└── doc_line/             # 文档
    ├── Paper_Studio_MVP_Roadmap.md
    └── Paper_Studio_Phase0_Technical_Solution.md
```

## PaperML 语法示例

```paper
@section Introduction

This paper proposes a novel framework @cite{ref2024}.

@figure{
  path = "framework.png"
  caption = "System overview"
  label = "fig:framework"
}

@equation{
  content = "E = mc^2"
  label = "eq:einstein"
}

@table{
  caption = "Results comparison"
  columns = ["Method", "ADE", "FDE"]
  rows = [
    ["Ours", "0.58", "1.02"],
    ["Baseline", "0.81", "1.52"]
  ]
}
```

## 技术栈

- **核心引擎**: Rust + pest (PEG parser)
- **WASM 绑定**: wasm-bindgen
- **Web 前端**: Vanilla JS + KaTeX
- **数学渲染**: KaTeX

## 开发命令

```bash
# 运行 Rust 测试
cd paper-core
cargo test

# 构建 WASM (需要 wasm-pack)
wasm-pack build --target web --out-dir ../web/pkg

# 本地开发服务器
cd web && python3 -m http.server 8080
```

## Phase 0 功能清单

- [x] PaperML 语法设计
- [x] pest 语法文件
- [x] AST 节点定义
- [x] Parser 实现
- [x] HTML Renderer
- [x] WASM 绑定代码
- [x] Web 前端界面
- [x] JS 回退解析器
- [x] 示例论文

## 后续计划

- **Phase 1**: Electron 桌面应用
- **Phase 2**: LaTeX 导出
- **Phase 3**: Word 导出
- **Phase 4**: AI 辅助功能

## License

MIT
