import re
from dataclasses import dataclass
from parser import BaseParser

from click import ClickException
from enums import Protocol


@dataclass
class ParsedURL:
    """
    Data extracted from the plane string url.
    """

    protocol: Protocol
    host: str
    port: int | None
    path: str | None
    query_params: str | None

    def __post_init__(self):
        if self.port is None:
            self.port = 80 if self.protocol == "http" else 443


class UrlParser(BaseParser):
    """
    Class responsible for extracting data from urls, such as `protocol`, `host`, etc.
    """

    @classmethod
    def parse(cls, url: str) -> ParsedURL:
        """
        Extracts usefull data from the url (e.g protocol, host, etc.)

        Args:
            url: url passed by the user.
        Returns:
            `ParsedURL`: Data extracted from the url in a structured form.
        """
        reg = re.search(
            r"(http|https)://([a-zA-Z0-9.]+)(:[0-9]+)?([a-zA-Z0-9./]+)?(\?.*)?", url
        )

        if reg is None:
            raise ClickException("Invalid URL format")

        protocol: Protocol = Protocol.UNSUPPORTED

        match reg.group(1):
            case "http":
                protocol = Protocol.HTTP
            case "https":
                protocol = Protocol.HTTPS

        port = reg.group(3)
        if port is not None:
            port = int(port[1:])

        return ParsedURL(protocol, reg.group(2), port, reg.group(4), reg.group(5))
