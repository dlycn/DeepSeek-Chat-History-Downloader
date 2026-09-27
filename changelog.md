# Changelog

## [0.4.0] — 2026-09-26

### Added

- **MCP Server 架构**: 从 CLI 工具完全重写为 MCP (Model Context Protocol) 服务器，基于 JSON-RPC 2.0，stdio 传输
  - `initialize` → 声明 serverInfo 和 capabilities
  - `tools/list` → 暴露 4 个工具供 AI 调用
  - `tools/call` → 路由到对应 handler
  - 遵守 MCP notification 语义（无 `id` 的请求不响应）
  - 未知方法返回 `-32601` 标准错误码
- **`zhihu_session` 工具**: 按标题关键词模糊搜索某个会话的完整详情，含所有对话轮次（USER/ASSISTANT），每段截断至 500 字
- **`zhihu_parse` 重写**: 不再做关键词匹配，改为输出会话摘要（标题+提问示例+统计）交给 AI 分析；默认展示最近 50 个会话
- **`zhihu_daily` 重写**: 并行输出用户会话上下文（最多 10 条）+ 知乎推荐问题全量列表（~200 条）+ AI 指令，由 AI 完成兴趣匹配和回答生成
- **`zhihu_init` 重写**: 支持增量更新（逐项覆盖非空参数），传入空值可清除对应字段
- **`logger.rs`**: `McpLogger` 结构化日志系统，基于 `tracing` + `tracing-subscriber` + `tracing-appender`，支持 daily rotation、自动截断长响应、最多保留 7 天日志
- **二进制重命名**: `aichdl` → `MPC_zhihu`，遵循 MCP 命名惯例
- **SKILL.md 全面重写**: 覆盖 MCP 协议说明、生命周期、4 个工具的完整参数表/输出格式/行为描述、数据模型、工作流程

### Changed

- `main.rs` CLI 入口完全重写为 MCP JSON-RPC 主循环
- `analyzer.rs`: 移除 10 维度关键词匹配逻辑，新增 `format_session_detail` / `format_zhihu_for_ai` 格式化函数
- `ds_parser.rs`: 新增 `parse_session_detail` (关键词搜索完整会话) / `count_all_sessions` (全局统计)
- `types.rs`: 移除 `UserQuestion` / `TopicProfile` / `DailyTask` 旧输出类型，新增 `SessionDetail` / `ConversationTurn`
- `settings.rs`: `zhihu_cookie` / `zhihu_xsrf` 从 `Option<String>` 改为 `String` 带默认空值；新增 `apply_and_save` 增量更新 / `save_settings` 持久化函数
- `Cargo.toml`: 新增 `tracing` / `tracing-subscriber` / `tracing-appender` workspace 依赖

### Removed

- 10 维度关键词兴趣画像匹配（`Rust编程` `游戏开发` 等），改为 AI 侧分析
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
- 知乎 API headers 不再硬编码敏感信息

### Docs

- 重写 `readme.md`，覆盖 Python 下载器 + Rust CLI 两部分
- 更新 `SKILL.md`，增加 `init` 命令说明和发布构建指引

---

## [0.2.0] — 2026-09-26

### Added

- **`ds_parser.rs`**: 遍历 `deepseek/` 目录下所有 JSON，提取每条 USER 消息的 `fragments[type=REQUEST].content`
- **`analyzer.rs`**: 10 维度关键词匹配构建用户兴趣画像，与知乎问题做关联匹配并输出建议回答角度
- **`types.rs`**: DS 对话 JSON 的完整反序列化结构体 + 输出类型（`UserQuestion`, `TopicProfile`, `DailyTask`）
- **`zhihu/mod.rs`**: 知乎 creator API 拉取推荐问题列表（id + title）
- **`config.rs`**: 知乎 API 请求所需复杂 Header 构建
- **`main.rs`**: CLI 入口，`parse` / `daily` 两个子命令
- **`SKILL.md`**: Trae Skill 定义，4 项任务清单（回答/提问/关注/点赞，后两项可延期）

### 10 个兴趣维度

`Rust编程` `游戏开发` `前端/GUI` `工具链/DevOps` `社会人文` `数学/科学` `生活消费` `AI/机器学习` `哲学/思辨` `Android/移动`

---

## [0.1.0] — 2025

### Added

- **`aichat_histdl/downloader.py`**: DeepSeek 网页版 API 对话下载器
- **`aichat_histdl/models.py`**: Session / ChatHistory 数据模型
- **`aichat_histdl/utils.py`**: `path_safe` 文件名安全处理
- 分页拉取会话列表
- 失败重试机制（409 → 自动重试最多 5 次）
- 本地缓存 `session_ids.json` / `error_session_ids.json`
- 环境变量 + 直接传参两种凭据配置方式