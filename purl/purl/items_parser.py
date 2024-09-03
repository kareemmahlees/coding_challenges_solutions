import re
from dataclasses import dataclass
from purl.parser import BaseParser
from typing import Dict, List
from purl.constants import DEFAULT_HEADERS


@dataclass
class RequestItems:
    """
    Class representing data parsed out of items passed from the user.
    """

    headers: Dict[str, str]
    data: Dict[str, str]

    def __post_init__(self):
        """
        Combines default headers with user sent headers.
        """
        headers = DEFAULT_HEADERS
        headers.update(self.headers)

        self.headers = headers


class ItemsParser(BaseParser):
    @classmethod
    def parse(cls, items: List[str] | None) -> RequestItems:
        """
        Extracts headers and json/form data from items.

        Args:
            items: items passed from the CLI.
        Returns:
            `RequestItems`: Parsed items in the form of strucutred data.
        """
        headers = {}
        data = {}

        if items is None:
            return RequestItems(headers, data)

        for item in items:
            header_reg = re.match(r"(.+):(.+)", item)

            if header_reg:
                headers.update({header_reg[1]: header_reg[2]})
                continue

            json_reg = re.match("(.+)=(.+)", item)

            if json_reg:
                data.update({json_reg[1]: json_reg[2]})
                continue

        return RequestItems(
            headers,
            data,
        )
