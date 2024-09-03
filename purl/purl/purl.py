from typing import Annotated, List, Optional

import typer
from enums import RequestMethod
from items_parser import ItemsParser
from request import RequestBuilder
from url_parser import UrlParser

app = typer.Typer()


@app.command()
def main(
    url: str = typer.Argument(help="Server url to send request to."),
    method: Annotated[
        RequestMethod, typer.Argument(help="Request method.")
    ] = RequestMethod.GET,
    items: Annotated[
        Optional[List[str]],
        typer.Argument(
            help="Headers and Json/Form data you which to send with the request.",
        ),
    ] = None,
    verbose: Annotated[
        bool,
        typer.Option("--verbose", "-v", help="Show more info about sent requests."),
    ] = False,
    offline: Annotated[
        bool, typer.Option(help="Only construct the request but don't send it.")
    ] = False,
    form: Annotated[
        bool,
        typer.Option("--form", "-f", help="Inidicate that sent data is form data."),
    ] = False,
):
    """
    Basically `curl`, but more powerfull.
    """
    parsed_url = UrlParser.parse(url)

    parsed_items = ItemsParser.parse(items)

    builder = RequestBuilder(parsed_url, method, parsed_items)

    builder.run(verbose, offline, form)


if __name__ == "__main__":
    app()
