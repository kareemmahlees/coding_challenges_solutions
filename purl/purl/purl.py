from parser import Parser
from typing import Annotated

import typer
from request import RequestBuilder, RequestMethod
from rich import print as pretty_print

app = typer.Typer()


@app.command()
def main(
    url: str,
    verbose: Annotated[bool, typer.Option()] = False,
    method: Annotated[RequestMethod, typer.Option("-X")] = RequestMethod.GET,
):
    parsed = Parser.parse(url)

    builder = RequestBuilder(parsed, method, {"Accept": "*/*", "Connection": "close"})

    res = builder.run()

    if verbose:
        for k, v in res.headers.items():
            pretty_print(f"> [bold green]{k}[/bold green]: {v}")

    pretty_print(res.text)


if __name__ == "__main__":
    app()
