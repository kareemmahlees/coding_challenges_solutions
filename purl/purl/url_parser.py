import re
from dataclasses import dataclass

from purl.parser import BaseParser

from click import ClickException
from purl.enums import Protocol


@dataclass
class ParsedURL:
    """
    Data extracted from the plane string url.
    """

    protocol: Protocol
    host: str
    port: int | None = None
    path: str | None = None
    query_params: str | None = None

    def __post_init__(self):
        if self.port is None:
            self.port = 80 if self.protocol == Protocol.HTTP else 443


class UrlParser(BaseParser):
    """
    Class responsible for extracting data from urls, such as `protocol`, `host`, etc.
    """

    @classmethod
    def try_localhost_parsing(cls, url: str) -> ParsedURL:
        localhost_reg = re.match(r"^(:[0-9]+)?(/[a-zA-Z0-9/]*)(\?.+)?", url)

        if localhost_reg is None:
            raise ClickException("Invalid URL format")

        port = localhost_reg.group(1)
        if port is not None:
            port = cls.normalize_port(port)

        return ParsedURL(
            Protocol.HTTP,
            "localhost",
            port,
            localhost_reg.group(2),
            localhost_reg.group(3),
        )

    @classmethod
    def normalize_port(cls, port: str) -> int:
        normalized_port = int(port[1:])
        return normalized_port

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
            parsed_url = cls.try_localhost_parsing(url)
            return parsed_url

        protocol: Protocol = Protocol.UNSUPPORTED

        match reg.group(1):
            case "http":
                protocol = Protocol.HTTP
            case "https":
                protocol = Protocol.HTTPS

        port = reg.group(3)
        if port is not None:
            port = cls.normalize_port(port)

        return ParsedURL(protocol, reg.group(2), port, reg.group(4), reg.group(5))
