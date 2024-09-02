from dataclasses import dataclass, field
from enum import StrEnum

from click import ClickException
from parser import ParsedURL
from typing import Dict, List, Self
import re

import requests
from requests import Response
from rich import print as pretty_print


class RequestMethod(StrEnum):
    GET = "GET"
    DELETE = "DELETE"
    POST = "POST"
    PUT = "PUT"


@dataclass
class RequestBuilder:
    parsed_data: ParsedURL
    method: RequestMethod
    data: str
    headers: Dict[str, str] = field(
        default_factory=lambda: {"Accept": "*/*", "Connection": "close"}
    )

    def append_headers(self, headers: List[str]) -> Self:
        for h in headers:
            header = re.match(r"(.+):\s+(.+)", h)
            if header is None:
                raise ClickException("Invalid header format")

            self.headers.update({header.group(1): header.group(2).strip(" ")})

        return self

    def contruct_request_url(self) -> str:
        req_url = f"{self.parsed_data.protocol}://{self.parsed_data.host}:{self.parsed_data.port}"
        if self.parsed_data.path:
            req_url += self.parsed_data.path
        if self.parsed_data.query_params:
            req_url += self.parsed_data.query_params

        return req_url

    def run(self, verbose: bool) -> Response:
        if verbose:
            print("")
            print(f"< connection to {self.parsed_data.host}")
            print(
                f"< Sending request GET {self.parsed_data.path} {self.parsed_data.protocol.value.upper()}/1.1"
            )
            print(f"< Host: {self.parsed_data.host}")
            for k, v in self.headers.items():
                pretty_print(f"< [bold green]{k}[/bold green]: {v}")
            print("< ")

        res = requests.request(
            self.method,
            self.contruct_request_url(),
            data=self.data,
            headers=self.headers,
        )
        return res
