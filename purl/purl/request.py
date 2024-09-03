from dataclasses import dataclass

import requests
from purl.enums import ContentType, RequestMethod
from purl.items_parser import RequestItems
from rich import print as pretty_print
from rich import print_json
from rich.console import Console
from rich.syntax import Syntax
from purl.url_parser import ParsedURL


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

    def print_outgoing_request(self):
        """
        Pretty print outgoing request headers and data.
        """
        pretty_print(
            f"[bold bright_green]{self.method}[/] {self.parsed_url.path} {self.parsed_url.protocol}/1.1"
        )
        for k, v in self.items.headers.items():
            pretty_print(f"[bold green]{k}[/]: ", end="")
            print(f"{v}")

        print("")
        print_json(data=self.items.data)
        print("\n")

    def print_incoming_response(self, res: requests.Response):
        """
        Pretty print incomming response headers and data.
        """
        pretty_print(f"[bold blue]{self.parsed_url.protocol}/1.1 {res.status_code}[/]")
        for k, v in res.headers.items():
            pretty_print(f"[bold green]{k}[/]: ", end="")
            print(f"{v}")

        print("")

        match res.headers["Content-type"]:
            case t if ContentType.Json in t:
                print_json(res.text, indent=3)

            case ContentType.Text:
                print(res.text)

            case ContentType.Html:
                console = Console()
                syntax = Syntax(res.text, "html")
                console.print(syntax)

            case _:
                print(res.text)

    def run(self, verbose: bool, offline: bool, form: bool):
        """
        Execute the constructed request.

        Args:
            verbose: wiether to print extra headers info to the terminal.
            offline: don't execute the request, just show me what will be sent.
            form: whether the passed data will be sent as form data or not.
        """
        if form:
            self.items.headers["Content-type"] = ContentType.Form

        if verbose or offline:
            self.print_outgoing_request()

            if offline:
                return

        res = requests.request(
            self.method,
            self.contruct_request_url(),
            json=self.items.data
            if ContentType.Json
            in self.items.headers[
                "Content-type"
            ]  # to account for `application/json; charset= utf8`
            else None,
            data=self.items.data if form else None,
            headers=self.items.headers,
        )

        self.print_incoming_response(res)
