import re
from dataclasses import dataclass
from enum import StrEnum
from typing import List

from click import ClickException


class ParserError(ClickException): ...


class Protocol(StrEnum):
    HTTP = "http"
    HTTPS = "https"
    UNSUPPORTED = "unsupported"


@dataclass
class ParsedURL:
    protocol: Protocol
    host: str
    port: int | None
    path: str | None
    query_params: List[str] | None
    method: str = "GET"

    def __post_init__(self):
        if self.port is None:
            self.port = 80 if self.protocol == "http" else 443

    def __str__(self) -> str:
        return "from str"


class Parser:
    @classmethod
    def parse(cls, url: str) -> ParsedURL:
        reg = re.search(
            r"(http|https)://([a-zA-Z0-9.]+)(:[0-9]+)?([a-zA-Z0-9./]+)?(\?.*)?", url
        )

        if reg is None:
            raise ParserError("Invalid URL format")

        protocol: Protocol = Protocol.UNSUPPORTED

        match reg.group(1):
            case "http":
                protocol = Protocol.HTTP
            case "https":
                protocol = Protocol.HTTPS

        port = reg.group(3)
        if port is not None:
            port = int(port[1:])

        query_params = reg.group(5)
        if query_params is not None:
            query_params = query_params[1:].split("&")

        return ParsedURL(protocol, reg.group(2), port, reg.group(4), query_params)
