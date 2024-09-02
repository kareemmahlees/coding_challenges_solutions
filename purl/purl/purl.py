from parser import Parser
from typing import Annotated, Optional

import typer
from request import RequestBuilder
from rich import print as pretty_print

app = typer.Typer()


@app.command()
def main(url: str, verbose: Annotated[Optional[bool], typer.Option()] = False):
    parsed = Parser.parse(url)

    builder = RequestBuilder(parsed, {"Accept": "*/*", "Connection": "close"})

    res = builder.run()

    if verbose:
        for k, v in res.headers.items():
            pretty_print(f"> [bold green]{k}[/bold green]: {v}")

    pretty_print(res.text)


if __name__ == "__main__":
    app()
