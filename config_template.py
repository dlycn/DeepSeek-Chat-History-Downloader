URL = "https://chat.deepseek.com"
TOKEN = (
    "..."  
)
COOKIE_STRING = "..."  

# 2. 构造请求头（完整模拟浏览器）
HEADERS = {
    "authorization": f"Bearer {TOKEN}",
    "cookie": COOKIE_STRING,
    "user-agent": "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/146.0.0.0 Safari/537.36 Edg/146.0.0.0",
    "x-app-version": "20241129.1",
    "x-client-platform": "web",
    "x-client-version": "1.7.1",
    "x-client-locale": "zh_CN",
    "referer": f"{URL}/",
    "origin": f"{URL}",
}