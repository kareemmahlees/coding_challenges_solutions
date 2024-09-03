from typing import Annotated, List, Optional

import typer
from enums import RequestMethod
from items_parser import ItemsParser
from request import RequestBuilder
from url_parser import UrlParser

app = typer.Typer()


@app.command()
def main(
    url: str = typer.Argument(),
    method: Annotated[RequestMethod, typer.Argument()] = RequestMethod.GET,
    items: Annotated[Optional[List[str]], typer.Argument()] = None,
    verbose: Annotated[bool, typer.Option()] = False,
    offline: Annotated[bool, typer.Option()] = False,
    form: Annotated[bool, typer.Option("--form", "-f")] = False,
):
    parsed_url = UrlParser.parse(url)

    parsed_items = ItemsParser.parse(items)

    builder = RequestBuilder(parsed_url, method, parsed_items)

    builder.run(verbose, offline, form)


if __name__ == "__main__":
    app()
