from dataclasses import dataclass, field
from typing import Any, Dict, List, Optional


@dataclass
class Session:
    id: str
    title: Optional[str]
    updated_at: Optional[float] = None
    raw: Dict[str, Any] = field(default_factory=dict, repr=False)


@dataclass
class ChatHistory:
    session_id: str
    raw: Dict[str, Any]