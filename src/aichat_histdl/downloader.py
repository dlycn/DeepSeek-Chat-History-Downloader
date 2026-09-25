import json
import logging
import os
import sys
import time
from typing import Dict, List, Optional

import requests

from .models import Session
from .utils import path_safe

logger = logging.getLogger(__name__)

_URL = "https://chat.deepseek.com"
_USER_AGENT = (
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) "
    "AppleWebKit/537.36 (KHTML, like Gecko) "
    "Chrome/146.0.0.0 Safari/537.36 Edg/146.0.0.0"
)
_SESSION_CACHE = "session_ids.json"
_ERROR_LOG = "error_session_ids.json"


class Downloader:
    def __init__(
        self,
        token: str = "",
        cookie: str = "",
        output_dir: str = "deepseek",
    ):
        if not logging.getLogger().handlers:
            logging.basicConfig(
                level=logging.INFO,
                format="%(asctime)s [%(levelname)s] %(message)s",
            )

        self._token = token or os.environ.get("DEEPSEEK_TOKEN", "")
        self._cookie = cookie or os.environ.get("DEEPSEEK_COOKIE", "")
        if not self._token or not self._cookie:
            logger.error(
                "缺少凭据。请通过以下任一方式提供:\n"
                "  Downloader(token=\"...\", cookie=\"...\")\n"
                "  环境变量: DEEPSEEK_TOKEN / DEEPSEEK_COOKIE"
            )
            sys.exit(1)

        self._output_dir = output_dir
        self._http = requests.Session()
        self._http.headers.update(self._build_headers())
        self._failed: List[str] = []

    def run(self) -> "Downloader":
        if not self._check_identity():
            logger.error("身份验证失败，退出")
            return self
        

        sessions = self._load_or_fetch_sessions()
        if not sessions:
            logger.warning("没有可处理的会话")
            return self

        logger.info("开始下载 %d 个会话", len(sessions))
        os.makedirs(self._output_dir, exist_ok=True)

        for i, s in enumerate(sessions, 1):
            if s.title is None:
                logger.info("[%d/%d] 跳过无标题会话 %s", i, len(sessions), s.id)
                continue
            logger.info("[%d/%d] %s", i, len(sessions), s.title)
            time.sleep(0.1)
            self._download_one(s)

        total = len(sessions)
        ok = total - len(self._failed)
        logger.info("下载完成: 成功 %d / 失败 %d / 总计 %d", ok, len(self._failed), total)
        if self._failed:
            with open(_ERROR_LOG, "w", encoding="utf-8") as f:
                json.dump(self._failed, f, ensure_ascii=False, indent=2)
            logger.info("失败列表已保存到 %s", _ERROR_LOG)

        return self

    def check(self) -> bool:
        return self._check_identity()

    def sessions(self) -> List[Session]:
        return self._load_or_fetch_sessions()

    def download(self, session: Session) -> Optional[dict]:
        data = self._fetch_history(session.id)
        if data is None:
            return None
        safe = path_safe(session.title) if session.title else "default"
        filepath = os.path.join(self._output_dir, f"chat_{safe}.json")
        os.makedirs(self._output_dir, exist_ok=True)
        with open(filepath, "w", encoding="utf-8") as f:
            json.dump(data, f, ensure_ascii=False, indent=2)
        logger.info("已保存: %s", filepath)
        return data

    # ---------- private ----------

    def _build_headers(self) -> Dict[str, str]:
        return {
            "authorization": f"Bearer {self._token}",
            "cookie": self._cookie,
            "user-agent": _USER_AGENT,
            "x-app-version": "20241129.1",
            "x-client-platform": "web",
            "x-client-version": "1.7.1",
            "x-client-locale": "zh_CN",
            "referer": f"{_URL}/",
            "origin": _URL,
        }

    def _check_identity(self) -> bool:
        resp = self._http.get(f"{_URL}/api/v0/users/current")
        if resp.status_code == 200:
            data = resp.json()
            logger.info("认证成功，用户: %s", data.get("email", "unknown"))
            return True
        logger.error("认证失败，状态码: %d", resp.status_code)
        return False

    def _load_or_fetch_sessions(self) -> List[Session]:
        if os.path.exists(_SESSION_CACHE):
            with open(_SESSION_CACHE, "r", encoding="utf-8") as f:
                raw = json.load(f)
            logger.info("从缓存加载了 %d 个会话", len(raw))
            return [
                Session(
                    id=r["id"], title=r.get("title"),
                    updated_at=r.get("updated_at"), raw=r,
                )
                for r in raw
            ]
        return self._fetch_sessions()

    def _fetch_sessions(self) -> List[Session]:
        sessions: List[Session] = []
        pointer: Optional[float] = None
        page = 0

        while True:
            page += 1
            url = f"{_URL}/api/v0/chat_session/fetch_page?lte_cursor.pinned=false"
            if pointer is not None:
                url += f"&lte_cursor.updated_at={pointer}"
            resp = self._http.get(url)
            if resp.status_code != 200:
                logger.error("获取会话列表失败，状态码: %d", resp.status_code)
                break
            data = resp.json()
            biz = data.get("data", {}).get("biz_data", {})
            chunk = biz.get("chat_sessions", [])
            logger.info("第 %d 页，获取 %d 个会话", page, len(chunk))

            for raw in chunk:
                sessions.append(
                    Session(
                        id=raw["id"], title=raw.get("title"),
                        updated_at=raw.get("updated_at"), raw=raw,
                    )
                )

            if not biz.get("has_more"):
                break
            if chunk:
                pointer = chunk[-1]["updated_at"]

        logger.info("共获取 %d 个会话", len(sessions))
        with open(_SESSION_CACHE, "w", encoding="utf-8") as f:
            json.dump([s.raw for s in sessions], f, ensure_ascii=False, indent=2)
        return sessions

    def _fetch_history(self, session_id: str) -> Optional[dict]:
        url = f"{_URL}/api/v0/chat/history_messages?chat_session_id={session_id}&cache_version=0"
        for attempt in range(1, 6):
            resp = self._http.get(url)
            if resp.status_code == 200:
                return resp.json()
            if resp.status_code == 409:
                logger.debug("session %s 返回 409，第 %d 次重试", session_id, attempt)
                time.sleep(1.0)
                continue
            logger.error("获取 %s 失败，状态码: %d", session_id, resp.status_code)
            return None
        logger.error("获取 %s 失败，已达最大重试次数", session_id)
        return None

    def _download_one(self, session: Session) -> None:
        logger.info("会话ID: %s, 标题: %s", session.id, session.title)
        data = self._fetch_history(session.id)
        if data is None:
            logger.warning("下载失败: %s", session.title)
            self._failed.append(session.id)
            return
        logger.info("获取成功，状态码: 200")
        safe = path_safe(session.title) if session.title else "default"
        filepath = os.path.join(self._output_dir, f"chat_{safe}.json")
        with open(filepath, "w", encoding="utf-8") as f:
            json.dump(data, f, ensure_ascii=False, indent=2)
        logger.info("已保存: %s", filepath)