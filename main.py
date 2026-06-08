import os
import re
import requests
import json
import logging
import time
from config import URL, HEADERS

logging.basicConfig(
    level=logging.INFO, format="%(asctime)s - %(levelname)s - %(message)s"
)


class Environment:
    def __init__(self,s=0,e=time.time(), cache_ver=0):
        self.url = URL
        self.session_ids = []
        self.extra_urls = [
            f"{URL}/api/v0/users/current",
            f"{URL}/api/v0/chat/history_messages",
            f"{URL}/api/v0/chat_session/fetch_page",
        ]
        self.headers = HEADERS
        self.cache_versions = cache_ver
        self.cache_interval = [s, e]

    def get_hist_info(self, chat_session_id: str):
        self.hist_info_url = f"{self.extra_urls[1]}?chat_session_id={chat_session_id}&cache_version={self.cache_versions}"
        return self.hist_info_url

    def get_fetch_info(self, pointer: float=None):
        self.fetch_page_url = f"{self.extra_urls[2]}?lte_cursor.pinned=false"
        if pointer:
            self.fetch_page_url = f"{self.fetch_page_url}&lte_cursor.updated_at={pointer}"
        print(f"获取会话分页信息成功，URL：{self.fetch_page_url}")
        return self.fetch_page_url

    def get_user_info(self):
        self.user_info_url = self.extra_urls[0]
        return self.user_info_url


class Bug:
    def __init__(self, environment: Environment):
        self.env = environment
        self.logger = logging.getLogger("root")
        self.session = requests.Session()
        self.session.headers.update(self.env.headers)
        self.error_session_ids = []
        self.identity_check = False
        self.time_s, self.time_e = env.cache_interval
        self.logger.info(f"初始化完成，时序区间：{self.time_s}-{self.time_e}开始获取会话列表")

    def check_identity(self):
        resp = self.session.get(self.env.get_user_info())
        if resp.status_code == 200:
            self.logger.info(f"认证成功，用户信息：{resp.json()}")
            self.identity_check = True
        else:
            self.logger.error(f"认证失败，状态码：{resp.status_code}")
            self.identity_check = False
    
    def get_sessions_ids(self):
        pointer = None
        while True:
            resp = self.session.get(self.env.get_fetch_info(pointer))
            if resp.status_code == 200:
                respdict = resp.json()
                database = respdict["data"]["biz_data"]
                self.logger.info(f"获取会话列表成功，会话列表长度：{len(database['chat_sessions'])}")
                self.env.session_ids.extend(database['chat_sessions'])
                if database['has_more']:
                    pointer = database['chat_sessions'][-1]['updated_at']
                    self.logger.info(f"最新会话插入时间：{pointer}")
                    continue
                if pointer < self.time_s:
                    pointer = self.time_s
                    self.logger.info(f"时序区间：{self.time_s}-{self.time_e}结束，退出循环")
                    break
                with open("session_ids.json", "w", encoding="utf-8") as f:
                    json.dump(self.env.session_ids, f, ensure_ascii=False, indent=4)
                break

            else:
                self.logger.error(f"获取会话列表失败，状态码：{resp.status_code}")
                self.env.session_ids = []
                break
        return self.env.session_ids

    def get_hist_infos(self):
        if self.identity_check:
            self.logger.info("认证成功")
        else:
            self.check_identity()
        if self.env.session_ids == []:
            self.logger.warning("会话列表为空，进行本地读取")
            if os.path.exists("session_ids.json"):
                with open("session_ids.json", "r", encoding="utf-8") as f:
                    self.env.session_ids = json.load(f)
            else:
                self.logger.warning("本地会话列表文件不存在，开始获取会话列表")
                self.env.session_ids = self.get_sessions_ids()
        self.logger.info(f"获取会话历史信息，会话列表长度：{len(self.env.session_ids)}")
        os.makedirs("deepseek", exist_ok=True)
        for session in self.env.session_ids:
            session_id = session['id']
            title = session['title']
            if title == None:
                continue
            self.logger.info(f"会话ID：{session_id}，标题：{title}")
            while True:
                time.sleep(0.1)
                error = False
                resp = self.session.get(self.env.get_hist_info(session_id))
                if resp.status_code == 200:
                    break                
                elif resp.status_code == 409:
                    time.sleep(1)
                    continue
                else:
                    error = True
                    break
            if error:
                self.logger.error(f"获取会话历史信息{session_id}失败，状态码：{resp.status_code}")
                self.error_session_ids.append(session_id)
                continue

            self.logger.info(f"获取会话历史信息成功，状态码：{resp.status_code}")
            title = self.path_safe(title)
            path = os.path.join("deepseek", f"chat_{title}.json")
            with open(path, "w", encoding="utf-8") as f:
                json.dump(resp.json(), f, ensure_ascii=False, indent=4)

        if self.error_session_ids != []:
            with open("error_session_ids.json", "w", encoding="utf-8") as f:
                json.dump(self.error_session_ids, f, ensure_ascii=False, indent=4)
            self.env.session_ids = self.error_session_ids
            self.logger.info(f"获取缺失会话历史信息")
            self.get_hist_infos()



    def path_safe(self,s: str, max_len=255) -> str:
        # 1. 替换禁止字符：保留字符 + 控制字符
        s = re.sub(r'[\\/:*?"<>|]', '_', s)                     # 替换保留字符
        # 2. 去除首尾空格和点
        s = ''.join(ch if ord(ch) >= 32 and ord(ch) != 127 else '_' for ch in s)
        s = s.strip(' .')
        if not s:
            s = 'default'
        # 4. 限制长度（尽量保留扩展名）
        if len(s) > max_len:
            if '.' in s:
                base, ext = s.rsplit('.', 1)
                base = base[:max_len - len(ext) - 1]
                s = base + '.' + ext
            else:
                s = s[:max_len]
        return s
        

if __name__ == "__main__":
    env = Environment()
    bug = Bug(env)
    bug.get_hist_infos()
