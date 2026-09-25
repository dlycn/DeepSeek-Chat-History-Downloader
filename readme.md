# AI Chat History DL + 知乎日常助手

DeepSeek 对话下载器（Python） + 知乎日常分析工具（Rust）。

```
┌─────────────────────────┐     ┌─────────────────────────┐
│  src/aichat_histdl/     │     │  src/*.rs (Rust CLI)    │
│  Python 下载器          │ ──► │  aichdl parse / daily   │
│                         │     │                         │
│  deepseek/chat_*.json   │     │  用户画像 → 知乎匹配     │
└─────────────────────────┘     └─────────────────────────┘
```

---

## 一、Python: DeepSeek 对话下载器

### 功能

- 自动获取当前用户的所有会话列表（支持分页）
- 逐个下载每个会话的完整聊天历史
- 会话标题经过文件名字符安全处理
- 失败重试（409→自动等 1s，最多 5 次）
- 本地缓存会话列表，避免重复拉取
- 失败会话 ID 记录到 `error_session_ids.json`

### 依赖

```bash
pip install requests
```

### 获取凭据

登录 [chat.deepseek.com](https://chat.deepseek.com)，F12 → Network，找任意 API 请求：

| 凭据 | 来源 |
|------|------|
| Token | 请求头 `authorization`，去掉 `Bearer ` 前缀 |
| Cookie | 请求头 `cookie` 完整值 |

### 使用

```python
from aichat_histdl import Downloader

Downloader(
    token="<token>",
    cookie="<cookie>",
).run()
```

详细用法见原有 Readme 下文，此处从略。

### 输出

| 文件 | 说明 |
|------|------|
| `deepseek/chat_{标题}.json` | 每个会话完整记录 |
| `session_ids.json` | 会话列表缓存 |
| `error_session_ids.json` | 下载失败的会话 |

---

## 二、Rust: 知乎日常助手 (`aichdl`)

### 安装 / 构建

```bash
# Debug
cargo build

# Release（独立分发的单文件二进制）
cargo build --release
# → ../target/release/aichdl.exe
```

### 命令

| 命令 | 说明 |
|------|------|
| `aichdl init` | 交互式配置：DS 目录、知乎 Cookie/xsrf，存 `settings.toml` |
| `aichdl parse [-d dir]` | 纯本地分析，输出用户画像 + 候选问题 |
| `aichdl daily [-d dir]` | 拉取知乎推荐问题 + 匹配兴趣（需先 `init`） |

### 首次配置

```bash
aichdl init
```

交互提示：
- **DeepSeek 对话目录** — 默认 `deepseek`（与 Python 下载器输出一致）
- **知乎 Cookie** — 多行输入，空行结束；不留空以启用 API 功能
- **知乎 x-xsrftoken** — Cookie 中 `_xsrf=` 字段值

凭据存入 `settings.toml`，以 `Option` 存储——**二进制中无硬编码凭据，无隐私泄露风险**。后续命令自动加载。

### 本地分析

```bash
aichdl parse
```

无需知乎凭据，纯本地。输出：
- 会话数 / 提问数
- 10 维度兴趣画像
- 可提取的知乎候选问题

### 完整日常

```bash
aichdl daily
```

1. 解析 DS 对话 JSON → 用户画像
2. 拉取知乎推荐问题（需有效 Cookie）
3. 关键词匹配 → 输出关联问题 + 建议角度

若未配置 Cookie，自动降级为纯本地分析。

### 数据流

```
deepseek/chat_*.json           知乎 API
       │                           │
       ▼                           ▼
  ds_parser.rs              zhihu/mod.rs
  (提取 USER 提问)          (推荐问题列表)
       │                           │
       └──────────┬────────────────┘
                  ▼
           analyzer.rs
      (关键词匹配 + 兴趣画像)
                  │
                  ▼
           main.rs (CLI)
        ┌────────┴────────┐
        ▼                 ▼
      parse             daily
   (本地报告)        (完整任务单)
```

### 10 个兴趣维度

`Rust编程` `游戏开发` `前端/GUI` `工具链/DevOps` `社会人文` `数学/科学` `生活消费` `AI/机器学习` `哲学/思辨` `Android/移动`

---

## 三、完整工作流

```bash
# 1. Python 下载 DS 对话
cd src
python -c "from aichat_histdl import Downloader; Downloader(token='...', cookie='...').run()"

# 2. Rust 首次配置（仅一次）
aichdl init

# 3. 日常使用
aichdl daily
```

---

## 四、项目结构

```
aichathistorydl/
├── Cargo.toml
├── settings.toml                    ← aichdl init 生成
├── src/
│   ├── main.rs                      ← CLI 入口
│   ├── settings.rs                  ← 交互式配置管理
│   ├── config.rs                    ← 知乎请求头构建
│   ├── ds_parser.rs                 ← DS JSON 解析
│   ├── analyzer.rs                  ← 画像 + 匹配
│   ├── types.rs                     ← 数据模型
│   ├── zhihu/mod.rs                 ← 知乎 API
│   ├── main.py                      ← Python 入口
│   └── aichat_histdl/               ← Python 下载器模块
│       ├── __init__.py
│       ├── downloader.py
│       ├── models.py
│       └── utils.py
└── deepseek/                        ← 下载的 DS 对话 JSON
    ├── chat_xxx.json
    └── ...
```

---

## 五、Python API 参考

### `Downloader`

| 参数 | 类型 | 默认值 | 说明 |
|------|------|--------|------|
| `token` | `str` | `""` | Bearer Token |
| `cookie` | `str` | `""` | Cookie 字符串 |
| `output_dir` | `str` | `"deepseek"` | JSON 输出目录 |

| 方法 | 返回值 | 说明 |
|------|--------|------|
| `.run()` | `Downloader` | 一键：验证→拉会话→全部下载 |
| `.check()` | `bool` | 验证凭据 |
| `.sessions()` | `list[Session]` | 获取会话列表 |
| `.download(s)` | `dict\|None` | 下载单个会话 |

### `path_safe(text, max_len=240)`

文件名安全处理，非法字符→`_`。

---

## 免责声明

仅供个人学习与备份使用。请勿用于非法用途，确保符合 DeepSeek 及知乎服务条款。