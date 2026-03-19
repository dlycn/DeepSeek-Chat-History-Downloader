# DeepSeek Chat History Downloader

此脚本用于从 DeepSeek 网页版 API 下载所有聊天会话的历史记录，保存为 JSON 文件。

## 功能

- 自动获取当前用户的所有会话列表（支持分页）
- 逐个下载每个会话的完整聊天历史
- 会话标题经过文件名字符安全处理，避免非法字符
- 失败重试机制，并将失败的会话 ID 记录到 `error_session_ids.json` 中
- 本地缓存会话列表 (`session_ids.json`)，避免重复拉取

## 使用方法

### 1. 克隆或下载本项目

```bash
git clone <your-repo-url>
cd <project-directory>
```

### 2. 安装依赖

需要 Python 3.6+ 以及 `requests` 库。

```bash
pip install requests
```

### 3. 配置认证信息

在 `config.py` 中填写你的 **Token** 和 **Cookie**。

- 打开 DeepSeek 网页版 (<https://chat.deepseek.com>)
- 登录后打开浏览器开发者工具（F12）
- 在“网络”(Network) 标签中，找到任意一个请求（如 `users/current`）
- 在请求头中复制 `authorization` 字段的值（不含 `Bearer ` 前缀）填入 `TOKEN`
- 复制整个 `cookie` 请求头的值填入 `COOKIE_STRING`

> ⚠️ **注意**：`config.py` 包含敏感信息，**请勿**将其提交到版本控制（已在 `.gitignore` 中默认忽略）。

### 4. 运行脚本

```bash
python main.py
```

脚本将依次执行：
- 验证身份（打印用户信息）
- 拉取会话列表，保存到 `session_ids.json`
- 遍历每个会话，下载其历史消息，保存到 `deepseek/chat_标题.json`

如果某些会话下载失败，它们的 ID 会被记录到 `error_session_ids.json`，脚本会重试这些失败的会话。

### 5. 输出文件

- `deepseek/chat_标题.json`：每个会话的完整响应（包含消息、时间戳等）
- `session_ids.json`：所有会话的元数据缓存
- `error_session_ids.json`：首次下载失败的会话 ID（如果有）

## 文件说明

```
.
├── .gitignore               # Git 忽略文件配置
├── config.py                # 认证配置（需手动填写）
├── main.py                  # 主程序
├── readme.md                # 本文档
├── session_ids.json         # 会话列表缓存（自动生成）
├── deepseek/                # 下载的 JSON 存放目录
└── error_session_ids.json   # 失败会话记录（自动生成）
```

## 注意事项

- 会话标题中可能包含 `\ / : * ? " < > |` 等 Windows 文件名非法字符，脚本已自动替换为下划线。
- 如果标题全部由非法字符组成或为空，会使用 `default` 作为文件名。
- 下载过程中如果遇到 `409` 状态码，脚本会等待 1 秒后重试。
- 请勿频繁运行脚本，避免触发反爬机制。

## 免责声明

本工具仅供个人学习、备份聊天记录使用。请勿用于非法用途，使用前请确保符合 DeepSeek 的服务条款。
