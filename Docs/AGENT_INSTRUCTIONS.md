# DocFlux: Agent Development Instructions

## 1. 项目愿景与定位
DocFlux 是一款桌面端应用程序，旨在为单个 Office 文档（优先支持 PPTX 和 DOCX）提供轻量级、基于 Git 的底层版本控制。
核心业务逻辑：通过接管 Office 文件的解压、XML 格式化（Pretty-print）与静默 Git 提交，解决本地多版本文档冗余的痛点。

## 2. 技术栈约束
* **前端 (GUI):** Vue 3 (Composition API) + TypeScript + Vite + TailwindCSS。
* **后端 (系统级控制):** Tauri (Rust)。
* **本地数据库:** SQLite (用于存储项目与 GUID 映射关系)。
* **代码规范:** * 结构清晰，严禁冗余代码。
  * 必须包含详尽的中文注释（解释核心逻辑和边界条件）。
  * 优先使用成熟的异步处理机制（前端 async/await，后端 Rust async/tokio）。

## 3. 核心目录与架构约定
本项目不采用分散在各个文档旁边的隐藏文件夹，而是采用集中的 Vault (保险箱) 架构。
* 数据存储根目录应设为操作系统的标准应用数据目录，允许用户自定义设置目录位置，例如在Windows下，如果检测到存在D盘，则默认数据目录设置为：`D:/DocFluxData/`，否则设置为 `%APPDATA%/DocFluxData/` 。
* 数据库文件位置：`DocFluxData/docflux_core.sqlite`。
* 各文档独立的 Git 仓库位置：`DocFluxData/repos/<GUID>/`。

## 4. 第一阶段任务：基础脚手架与目录构建 (当前需执行)
请 Agent 严格按照以下步骤完成初始构建，每完成一步请在输出中向开发者确认：

**步骤 1: 初始化 Tauri + Vue 项目**
* 在当前目录初始化 Tauri 脚手架，使用 Vue + TypeScript 模板。
* 确保 `package.json` 和 `tauri.conf.json` 配置正确，项目名称设定为 `DocFlux`。

**步骤 2: 安装前端核心依赖**
* 安装 TailwindCSS 并完成基础配置（`tailwind.config.js`、`postcss.config.js`、引入全局 CSS）。
* 安装 Vue Router (用于多页面视图管理) 和 Pinia (全局状态管理)。

**步骤 3: 编写 Rust 后端初始化逻辑 (docflux_core)**
* 在 `src-tauri/src/main.rs` 及相关模块中，编写应用启动时的初始化检查机制。
* 核心逻辑：检测宿主系统是否存在 `DocFluxData` 目录，若不存在则自动创建 `DocFluxData` 及其子目录 `repos`。
* 编写一个可通过 Tauri invoke 调用的测试函数：`fn greet_docflux() -> String`，并在 Vue 前端主页调用此函数，确保前后端通信成功。

**步骤 4: 清理模板**
* 删除 Tauri 默认模板中不相关的 Demo 代码（如默认的 Logo 和计数器）。
* 在前端构建一个简洁的占位主页，包含“胖哥文溯 DocFlux - 核心引擎已启动”字样以及后端通信状态显示。

## 5. Agent 工作准则
* **严格执行，拒绝发散：** 未经明确要求，不要引入额外的框架或“创意”功能。
* **深度解析，拒绝含糊：** 如果在配置 Rust 跨平台路径读取（尤其是 AppData 目录获取）时遇到系统级差异，请在注释和回复中给出详细原理解析。
* 当需要执行终端命令（如 `npm install` 或 `cargo add`）时，请直接给出完整的命令行代码块。