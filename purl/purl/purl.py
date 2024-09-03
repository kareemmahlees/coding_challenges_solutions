from parser import UrlParser
from typing import Annotated, List, Optional

import typer
from request import RequestBuilder, RequestMethod

app = typer.Typer()


@app.command()
def main(
    method: Annotated[RequestMethod, typer.Argument()] = RequestMethod.GET,
    url: str = typer.Argument(),
    headers: Annotated[Optional[List[str]], typer.Option("--header", "-H")] = None,
    data: Annotated[Optional[str], typer.Option("--data", "-d")] = None,
    verbose: Annotated[bool, typer.Option()] = True,
    offline: Annotated[bool, typer.Option()] = False,
):
    parsed = UrlParser.parse(url)

    builder = RequestBuilder(parsed, method, data)

    if headers:
        builder.append_headers(headers)

    builder.run(verbose, offline)


if __name__ == "__main__":
    app()
