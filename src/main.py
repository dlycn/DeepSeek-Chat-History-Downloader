"""
DeepSeek 聊天记录下载模块。

公开 API 只有两个东西:
    Downloader   — 下载器
    path_safe    — 文件名安全处理

───────────────────────────────────────────
用法 1: 显式传凭据（推荐）
───────────────────────────────────────────

    from aichat_histdl import Downloader

    Downloader(token="xxx", cookie="xxx").run()

───────────────────────────────────────────
用法 2: 环境变量
───────────────────────────────────────────

    set DEEPSEEK_TOKEN=xxx
    set DEEPSEEK_COOKIE=xxx

    from aichat_histdl import Downloader

    Downloader().run()

───────────────────────────────────────────
用法 3: 分步控制
───────────────────────────────────────────

    from aichat_histdl import Downloader, path_safe

    dl = Downloader(token="xxx", cookie="xxx")

    dl.check()                  # 验证身份

    for s in dl.sessions():     # 获取会话列表
        dl.download(s)          # 逐个下载
"""

if __name__ == "__main__":
    print(
        "aichat_histdl 是模块，不是一键脚本。\n"
        "请阅读源码中的使用示例，自行编写调用代码。\n"
        "这是刻意的设计：你需要知道每一步在做什么。"
    )
    from aichat_histdl import Downloader
    import config


    Downloader(token=config.TOKEN, cookie=config.COOKIE_STRING).run()
