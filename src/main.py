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

    TOKEN = (
        "vZo78lz4Ztz8Aa0vnfBFCRqM4YHETX+xQ6sbyD2q4mx2RUn5ufHTJF6yXGEcDb2T"  # 替换为最新的
    )
    COOKIE_STRING = "smidV2=20251118024359c361ae887274b6c4698b8d53ecb2a03b007d1c54904c8b4a0; .thumbcache_6b2e5483f9d858d7c661c5e276b6a6ae=C8Vgz2OpkSxr+HCKKbP9jD/Ci6FqC/XliaubAP4HdHyfnMgdP7iaoabG4k368rE4LrwBvm/HP1oMYYktX3CQ/A%3D%3D; intercom-device-id-guh50jw4=28285566-7731-4758-b4ba-489b53eb8db7; HWWAFSESID=e49cc2626acbdf84144; HWWAFSESTIME=1773638457742; ds_session_id=c786dd2be64e456484d64e62162c8651"
    Downloader(token=TOKEN, cookie=COOKIE_STRING).run()
