from parser import Parser
from typing import Annotated, List, Optional

import typer
from request import RequestBuilder, RequestMethod

app = typer.Typer()


@app.command()
def main(
    url: str,
    data: Annotated[str, typer.Option("--data", "-d")],
    headers: Annotated[Optional[List[str]], typer.Option("--header", "-H")],
    verbose: Annotated[bool, typer.Option()] = True,
    method: Annotated[RequestMethod, typer.Option("-X")] = RequestMethod.GET,
    offline: Annotated[bool, typer.Option()] = False,
):
    parsed = Parser.parse(url)

    builder = RequestBuilder(parsed, method, data)

    if headers:
        builder.append_headers(headers)

    builder.run(verbose, offline)


if __name__ == "__main__":
    app()
