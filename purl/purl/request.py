from dataclasses import dataclass
from enum import StrEnum
from parser import ParsedURL
from typing import Dict

import requests
from requests import Response
from rich import print


class RequestMethod(StrEnum):
    GET = "GET"


@dataclass
class RequestBuilder:
    parsed_data: ParsedURL
    headers: Dict[str, str]
    method: RequestMethod = RequestMethod.GET

    def contruct_request_url(self) -> str:
        req_url = f"{self.parsed_data.protocol}://{self.parsed_data.host}:{self.parsed_data.port}"
        if self.parsed_data.path:
            req_url += self.parsed_data.path
        if self.parsed_data.query_params:
            req_url += self.parsed_data.query_params

        return req_url

    def run(self) -> Response:
        print(f"< connection to {self.parsed_data.host}")
        print(
            f"< Sending request GET {self.parsed_data.path} {self.parsed_data.protocol.value.upper()}/1.1"
        )
        print(f"< Host: {self.parsed_data.host}")
        print("< Accept: */*")
        print("< Connection: close")
        print("< ")

        res = requests.request(
            self.method,
            self.contruct_request_url(),
            headers=self.headers,
        )
        return res
