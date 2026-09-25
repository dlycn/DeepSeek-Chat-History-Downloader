import re


def path_safe(s: str, max_len: int = 240) -> str:
    s = re.sub(r'[\\/:*?"<>|]', '_', s)
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