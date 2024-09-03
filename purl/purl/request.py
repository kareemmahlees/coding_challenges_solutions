import re
from dataclasses import dataclass, field
from enum import StrEnum
from parser import ParsedURL
from typing import Dict, List, Self

import requests
from click import ClickException
from rich import print as pretty_print, print_json


class RequestMethod(StrEnum):
    """
    Enum represeting supported http methods by `purl`.
    """

    GET = "GET"
    DELETE = "DELETE"
    POST = "POST"
    PUT = "PUT"


@dataclass
class RequestBuilder:
    """
    A builder that handles request related operations, including running the request.
    """

    parsed_data: ParsedURL
    method: RequestMethod
    data: str | None
    """
    Data sent as the body of a `POST` or `PUT` rquest. 
    """
    headers: Dict[str, str] = field(
        default_factory=lambda: {"Accept": "*/*", "Connection": "close"}
    )

    def append_headers(self, headers: List[str]) -> Self:
        """
        Extend the default headers with a list of headers passed from the user.

        Args:
            headers: list of headers passed from the user calling the `--header` flag multible times.
        Returns:
            The same object with mutated headers.
        Raises:
            `ClickException`: If headers are mallformed.
        """
        for h in headers:
            header = re.match(r"(.+):\s+(.+)", h)
            if header is None:
                raise ClickException("Invalid header format")

            self.headers.update({header.group(1): header.group(2).strip(" ")})

        return self

    def contruct_request_url(self) -> str:
        """
        Constructs the request url from the preparsed data.

        Returns:
            request_url
        """
        req_url = f"{self.parsed_data.protocol}://{self.parsed_data.host}:{self.parsed_data.port}"
        if self.parsed_data.path:
            req_url += self.parsed_data.path
        if self.parsed_data.query_params:
            req_url += self.parsed_data.query_params

        return req_url

    def run(self, verbose: bool, offline: bool):
        """
        Execute the constructed request.

        Args:
            verbose: wiether to print extra headers info to the terminal.
            offlien: don't execute the request, just show me what will be sent.
        """
        if verbose or offline:
            pretty_print(
                f"[bold bright_green]{self.method}[/] {self.parsed_data.path} {self.parsed_data.protocol}/1.1"
            )
            for k, v in self.headers.items():
                pretty_print(f"[bold green]{k}[/]: {v}")
            print("")
            print_json(self.data)
            print("\n")

            if offline:
                return

        res = requests.request(
            self.method,
            self.contruct_request_url(),
            data=self.data,
            headers=self.headers,
        )

        pretty_print(f"[bold blue]{self.parsed_data.protocol}/1.1 {res.status_code}[/]")
        for k, v in res.headers.items():
            pretty_print(f"[bold green]{k}[/]: {v}")

        print("")
        print_json(res.text, indent=3)
