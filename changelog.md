# Changelog

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