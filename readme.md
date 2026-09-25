# DeepSeek Chat History Downloader

从 DeepSeek 网页版 API 下载所有聊天会话的历史记录，保存为 JSON 文件。

## 功能

- 自动获取当前用户的所有会话列表（支持分页）
- 逐个下载每个会话的完整聊天历史
- 会话标题经过文件名字符安全处理，避免非法字符
- 失败重试机制（409 自动重试最多 5 次）
- 本地缓存会话列表 (`session_ids.json`)，避免重复拉取
- 失败会话 ID 记录到 `error_session_ids.json`

## 项目结构

```
aichathistorydl/
├── src/
│   ├── main.py                     # 模块文档 + 直接运行示例
│   ├── main.rs                     # Rust 占位（workspace 成员）
│   └── aichat_histdl/
│       ├── __init__.py             # 导出 Downloader / path_safe
│       ├── downloader.py           # 核心下载逻辑
│       ├── models.py               # 数据模型（Session / ChatHistory）
│       └── utils.py                # 工具函数（path_safe）
├── Cargo.toml                      # Rust workspace 配置
├── .python-version                 # Python 3.12
└── .gitignore
```

## 依赖

- **Python 3.12+**
- `requests`

```bash
pip install requests
```

## 获取凭据

登录 [chat.deepseek.com](https://chat.deepseek.com)，打开浏览器开发者工具（F12）→ 网络（Network）标签，找到任意 API 请求，从中提取：

| 凭据 | 来源 |
|------|------|
| **Token** | 请求头 `authorization` 的值，去掉 `Bearer ` 前缀 |
| **Cookie** | 请求头 `cookie` 的完整值 |

## 使用方式

> **设计说明**：`aichat_histdl` 是一个 Python **模块**，不是一键脚本。你需要自行编写调用代码——这是刻意设计的，确保你知道每一步在做什么。

### 方式一：显式传参（推荐）

```python
from aichat_histdl import Downloader

Downloader(
    token="<你的 token>",
    cookie="<你的 cookie>",
    output_dir="deepseek",      # 可选，默认 deepseek/
).run()
```

### 方式二：环境变量

```bash
# Windows PowerShell
$env:DEEPSEEK_TOKEN = "<你的 token>"
$env:DEEPSEEK_COOKIE = "<你的 cookie>"

# Linux / macOS
export DEEPSEEK_TOKEN="<你的 token>"
export DEEPSEEK_COOKIE="<你的 cookie>"
```

```python
from aichat_histdl import Downloader

Downloader().run()
```

### 方式三：分步控制

```python
from aichat_histdl import Downloader

dl = Downloader(token="...", cookie="...")

dl.check()                      # 验证身份
for s in dl.sessions():         # 遍历会话列表
    dl.download(s)              # 逐个下载
```

### 方式四：直接运行 `main.py`（快速体验）

`src/main.py` 的 `__main__` 块中硬编码了示例凭据（已失效），替换为你的凭据后可以直接运行：

```bash
cd src
python main.py
```

## 输出

| 文件 | 说明 |
|------|------|
| `deepseek/chat_{标题}.json` | 每个会话的完整聊天记录 |
| `session_ids.json` | 会话列表缓存（自动生成） |
| `error_session_ids.json` | 下载失败的会话 ID（自动生成） |

## API

### `Downloader`

| 参数 | 类型 | 默认值 | 说明 |
|------|------|--------|------|
| `token` | `str` | `""` | Bearer Token，为空则读环境变量 `DEEPSEEK_TOKEN` |
| `cookie` | `str` | `""` | Cookie 字符串，为空则读环境变量 `DEEPSEEK_COOKIE` |
| `output_dir` | `str` | `"deepseek"` | JSON 输出目录 |

| 方法 | 返回值 | 说明 |
|------|--------|------|
| `.run()` | `Downloader` | 一键执行：验证 → 拉取会话 → 下载全部 |
| `.check()` | `bool` | 验证凭据是否有效 |
| `.sessions()` | `list[Session]` | 获取会话列表 |
| `.download(session)` | `dict \| None` | 下载单个会话的完整记录 |

### `path_safe(text, max_len=240)`

将字符串转换为合法的文件名，替换非法字符为 `_`。

## 注意事项

- 会话标题中的 `\ / : * ? " < > |` 等字符会被替换为下划线
- 空标题会话使用 `default` 作为文件名
- 遇到 409 状态码会自动等待 1 秒后重试（最多 5 次）
- 请勿频繁运行，避免触发反爬机制

## 敏感数据存储建议

**`.crt` 文件不适合存储敏感数据。** `.crt` 是 X.509 数字证书的标准扩展名，设计用途是存放**公钥证书**（本身就是公开信息）。用它存敏感数据有几个问题：

| 问题 | 说明 |
|------|------|
| 语义误导 | 看到 `.crt` 会以为是证书，容易误判 |
| 容易泄露 | 证书文件通常被提交到仓库，不在 `.gitignore` 习惯中 |
| 无加密 | 改扩展名不等于加密，数据仍然是明文 |

### 推荐方案

| 方式 | 适用场景 |
|------|----------|
| 直接传参 | 临时使用、脚本内调用 |
| 环境变量 | CI/CD、容器化环境 |
| `config.py`（加入 .gitignore） | 本地开发，写入后不会被提交 |
| `.env` 文件 + `python-dotenv` | 最通用的本地配置方案 |

**示例 — `config.py`：**

```python
# config.py（已在 .gitignore 中，不会被提交到 Git）
TOKEN = "your-token-here"
COOKIE = "your-cookie-here"
```

```python
# 调用时导入
from config import TOKEN, COOKIE
from aichat_histdl import Downloader

Downloader(token=TOKEN, cookie=COOKIE).run()
```

## 免责声明

本工具仅供个人学习、备份聊天记录使用。请勿用于非法用途，使用前请确保符合 DeepSeek 的服务条款。