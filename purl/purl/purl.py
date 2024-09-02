from parser import Parser

import typer
from rich import print

app = typer.Typer()


@app.command()
def main(url: str):
    res = Parser.parse(url)
    print(f"connection to {res.host}")
    print(f"Sending request {res.method} {res.path} {res.protocol.value.upper()}/1.1")
    print(f"Host: {res.host}")
    print("Accept: */*")


if __name__ == "__main__":
    app()
