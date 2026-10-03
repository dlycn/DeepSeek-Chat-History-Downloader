import re
import json


def path_safe(s: str, max_len: int = 240) -> str:
    s = re.sub(r'[\\/:*?"<>|]', "_", s)
    s = "".join(ch if ord(ch) >= 32 and ord(ch) != 127 else "_" for ch in s)
    s = s.strip(" .")
    if not s:
        s = "default"
    if len(s) > max_len:
        if "." in s:
            base, ext = s.rsplit(".", 1)
            base = base[: max_len - len(ext) - 1]
            s = f"{base}.{ext}"
        else:
            s = s[:max_len]
    return s


def text_filter(path: str) -> str:
    json_data = json.load(open(path, "r", encoding="utf-8"))
    out = "# 聊天记录过滤\n"
    filter_text: list = json_data["data"]["biz_data"]["chat_messages"]
    for msg in filter_text:
        if "fragments" not in msg:
            continue
        fragment: list = msg["fragments"]

        for part in fragment:
            part_dict: dict = part
            if "type" in part_dict and "content" in part_dict:
                out += f"## {part_dict.get("type")}" + "\n"
                out += f"{part_dict.get("content")}" + "\n"
    with open("filter.md", "w", encoding="utf-8") as f:
        f.write(out)
    return out
