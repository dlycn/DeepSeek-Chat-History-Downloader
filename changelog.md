# Changelog

## [0.4.2] — 2026-09-29

### Changed

- **模块化拆分**: `main.rs` 拆分为三层——协议层 (`main.rs` + `mcp.rs`) + 业务层 (`handlers.rs`) + 领域层 (其余模块)，职责清晰
- **`types.rs` 大幅精简**: 移除 6 个 DS JSON 反序列化 struct（`ChatHistory` / `ChatData` / `BizData` / `ChatSession` / `ChatMessage` / `Fragment`），仅保留 3 个领域类型（`SessionSummary` / `SessionDetail` / `ConversationTurn`）
- **`ds_parser.rs` 重写**: 弃用 serde struct 反序列化，改用 `serde_json::Value` + `dig()` 路径导航，JSON 字段缺失时不 panic
- **`config.rs` → `zhihu/mod.rs`**: 知乎请求头构建合并到 `zhihu::Api` struct，`build_headers` / `init_urls` 内部化
- **工具定义 DRY**: `json!()` 手写块替换为 `ToolDef` builder（`tool().prop().required().build()`），加新工具只需声明式链式调用

### Added

- **`zhihu_fetch` 工具**: 独立拉取知乎推荐问题列表（~200 条），缓存至 exe 同级 `zhihutask.md`，与 `zhihu_daily` 解耦
- **`ping` 响应**: MCP 协议新增 `ping` method 处理，返回空 `result: {}`
- **`handlers.rs`**: 新建业务层模块，集中管理 5 个 handler（`zhihu_init` / `zhihu_parse` / `zhihu_session` / `zhihu_fetch` / `zhihu_daily`）及工具定义
- **`mcp.rs`**: 新建协议层模块，封装 MCP 初始化/工具列表/调用/错误响应，暴露 `McpServer::route()`
- **`settings.rs`**: `base_dir()` 改为 `pub`，供外部模块获取 exe 目录路径

---

## [0.4.0] — 2026-09-26

### Added

- **MCP Server 架构**: 从 CLI 工具完全重写为 MCP (Model Context Protocol) 服务器，基于 JSON-RPC 2.0，stdio 传输
  - `initialize` → 声明 serverInfo 和 capabilities
  - `tools/list` → 暴露工具供 AI 调用
  - `tools/call` → 路由到对应 handler
  - 遵守 MCP notification 语义（无 `id` 的请求不响应）
  - 未知方法返回 `-32601` 标准错误码
- **`zhihu_session` 工具**: 按标题关键词模糊搜索某个会话的完整详情，含所有对话轮次（USER/ASSISTANT），每段截断至 500 字
- **`zhihu_parse` 重写**: 不再做关键词匹配，改为输出会话摘要（标题+提问示例+统计）交给 AI 分析；默认展示最近 50 个会话
- **`zhihu_daily` 重写**: 并行输出用户会话上下文（最多 10 条）+ 知乎推荐问题全量列表（~200 条）+ AI 指令，由 AI 完成兴趣匹配和回答生成
- **`zhihu_init` 重写**: 支持增量更新（逐项覆盖非空参数），传入空值可清除对应字段
- **`logger.rs`**: `McpLogger` 结构化日志系统，基于 `tracing` + `tracing-subscriber` + `tracing-appender`，支持 daily rotation、自动截断长响应、最多保留 7 天日志
- **二进制重命名**: `aichdl` → `MCP_zhihu`，遵循 MCP 命名惯例
- **SKILL.md 全面重写**: 覆盖 MCP 协议说明、生命周期、工具的完整参数表/输出格式/行为描述、数据模型、工作流程

### Changed

- `main.rs` CLI 入口完全重写为 MCP JSON-RPC 主循环
- `analyzer.rs`: 移除 10 维度关键词匹配逻辑，新增 `format_session_detail` / `format_zhihu_for_ai` 格式化函数
- `ds_parser.rs`: 新增 `parse_session_detail` (关键词搜索完整会话) / `count_all_sessions` (全局统计)
- `types.rs`: 移除 `UserQuestion` / `TopicProfile` / `DailyTask` 旧输出类型，新增 `SessionDetail` / `ConversationTurn`
- `settings.rs`: `zhihu_cookie` / `zhihu_xsrf` 从 `Option<String>` 改为 `String` 带默认空值；新增 `apply_and_save` 增量更新 / `save_settings` 持久化函数
- `Cargo.toml`: 新增 `tracing` / `tracing-subscriber` / `tracing-appender` workspace 依赖

### Removed

- 10 维度关键词兴趣画像匹配，改为 AI 侧分析
- CLI `parse` / `daily` / `init` 子命令（由 MCP tools 替代）
- `TopicProfile` / `UserQuestion` / `DailyTask` 废弃类型

## [0.3.0] — 2026-09-26

### Added

- **交互式配置系统** (`aichdl init`): 首次运行时交互式提示输入 DS 目录、知乎 Cookie/xsrf，存入 `settings.toml` 自动加载，后续命令无需重复配置
- **凭据安全**: `zhihu_cookie` / `zhihu_xsrf` 改为 `Option<String>`，二进制中无任何硬编码凭据
- **优雅降级**: `daily` 命令在未配置知乎凭据时自动退化为纯本地分析，不报错、不 panic
- **`settings.rs`**: TOML 序列化配置管理，支持多行 Cookie 粘贴输入
- **`config.rs` 重构**: `build_headers(cookie, xsrf)` 接受参数而非硬编码
- **`zhihu/mod.rs` 重构**: `get_from_id(url, headers)` 接受外部传入 headers

### Changed

- `main.rs` CLI 从 2 条命令扩展为 3 条 (`init` / `parse` / `daily`)
- `Cargo.toml` 新增 `toml` workspace 依赖