from parser import Parser

import typer
from request import RequestBuilder
from rich import print as pretty_print

app = typer.Typer()


@app.command()
def main(url: str):
    parsed = Parser.parse(url)

    builder = RequestBuilder(parsed, {"Accept": "*/*", "Connection": "close"})

    res = builder.run()

    for k, v in res.headers.items():
        pretty_print(f"[bold green]{k}[/bold green]: {v}")

    pretty_print(res.text)


if __name__ == "__main__":
    app()
