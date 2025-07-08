<div align="center">
    <img width="200px" height="200px" src="./logo.svg" />
   	<br />
    <h1 style="margin: 10px">
        🌟 Lumin
    </h1>
    <p>简单的跨平台桌面应用程序目标，基于Rust和Slint</p>
</div>

## 🐳 核心特性
- ⚡ **基于Rust**：内存安全、零成本抽象等等
- 🖥️ **Slint UI 框架**：声明式 UI、响应式设计、轻量
- 🌐 **原生跨平台**：支持 Windows/macOS/Linux 一键构建
- 📦 **开箱即用的工具链**：
  - 集成日志系统（env_logger）
  - 发布优化配置（LTO + strip）
- 🔄 **自动构建Slint**：自动编译 `ui/` 目录下所有 .slint 文件

## 🧑‍💻 快速开始
安装 [Rust 工具链](https://www.rust-lang.org/tools/install)

### 创建项目
在Github页面中点击“Use this template”按钮，然后点击“Create a new repository”按钮，并按照提示操作。
或者，你可以Clone仓库。

### 运行应用程序
```sh
cargo run # 运行应用程序
```

### 构建应用程序
```sh
cargo build --release # 构建应用程序
```

### 交叉编译与打包
本模板不提供交叉编译与打包的默认配置，你可以根据需要自行配置。

## 🖊️ VSCode 集成
模板推荐使用 [VSCode](https://code.visualstudio.com/) 作为开发环境，并安装以下扩展以获得LSP Server支持：
- `slint.slint` - Slint 语言支持
- `rust-lang.rust-analyzer` - Rust 语言支持

### 代码格式化
如果您安装了`slint`与`rust-analyzer`插件，VSCode 将在保存时自动格式化代码。
如果您不喜欢自动格式化，或者想要修改一些配置，可以在`.vscode/settings.json`中调整或禁用它：
```diff
-  "editor.formatOnSave": true,
-  "editor.formatOnPaste": true,
```
```diff
-  "[rust]": {
-      "editor.formatOnSave": true,
-      "editor.defaultFormatter": "rust-lang.rust-analyzer",
-      "editor.tabSize": 4,
-      "editor.insertSpaces": true
-  },
```
```diff
-  "[slint]": {
-    "editor.formatOnSave": true,
-    "editor.defaultFormatter": "Slint.slint",
-    "editor.tabSize": 4,
-    "editor.insertSpaces": true
-  }
```

## 📜 许可证
本项目采用 **[MIT-0 许可证](https://opensource.org/license/mit-0)** - 你可以：
- 自由使用、修改和分发本作品
- 用于商业或非商业目的
- **无需保留版权声明或许可证副本**
- 无任何担保或责任
