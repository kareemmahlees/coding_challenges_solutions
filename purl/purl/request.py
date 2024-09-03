from dataclasses import dataclass

import requests
from enums import RequestMethod
from items_parser import RequestItems
from rich import print as pretty_print, print_json
from rich.console import Console
from rich.syntax import Syntax
from url_parser import ParsedURL
from enums import ContentType


@dataclass
class RequestBuilder:
    """
    A builder that handles request related operations, including running the request.
    """

    parsed_url: ParsedURL
    method: RequestMethod
    items: RequestItems

    def contruct_request_url(self) -> str:
        """
        Constructs the request url from the preparsed data.

        Returns:
            request_url
        """
        req_url = f"{self.parsed_url.protocol}://{self.parsed_url.host}:{self.parsed_url.port}"
        if self.parsed_url.path:
            req_url += self.parsed_url.path
        if self.parsed_url.query_params:
            req_url += self.parsed_url.query_params

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
                f"[bold bright_green]{self.method}[/] {self.parsed_url.path} {self.parsed_url.protocol}/1.1"
            )
            # Print outgoing headers
            for k, v in self.items.headers.items():
                pretty_print(f"[bold green]{k}[/]: {v}")
            print("")
            print_json(data=self.items.data)
            print("\n")

            if offline:
                return

        res = requests.request(
            self.method,
            self.contruct_request_url(),
            json=self.items.data
            if self.items.headers["Content-type"] == "application/json"
            else None,
            # TODO: support form data
            # data=self.items.data,
            headers=self.items.headers,
        )

        # Print upcomming headers
        pretty_print(f"[bold blue]{self.parsed_url.protocol}/1.1 {res.status_code}[/]")
        for k, v in res.headers.items():
            pretty_print(f"[bold green]{k}[/]: {v}")

        print("")

        match res.headers["Content-type"]:
            case ContentType.Json:
                print_json(res.text, indent=3)

            case ContentType.Text:
                print(res.text)

            case ContentType.Html:
                console = Console()
                syntax = Syntax(res.text, "html")
                console.print(syntax)
