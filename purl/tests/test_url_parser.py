from click import ClickException
import pytest

from purl.url_parser import ParsedURL, UrlParser
from purl.enums import Protocol


@pytest.mark.parametrize(
    "test_input,expected,raises",
    [
        ("http://github.com", ParsedURL(Protocol.HTTP, "github.com", 80), False),
        ("https://github.com", ParsedURL(Protocol.HTTPS, "github.com", 443), False),
        (
            "https://github.com/api/client",
            ParsedURL(Protocol.HTTPS, "github.com", path="/api/client"),
            False,
        ),
        (
            "https://github.com?page=1",
            ParsedURL(Protocol.HTTPS, "github.com", query_params="?page=1"),
            False,
        ),
        ("github.com", None, True),
        ("http", None, True),
        (":5000/foo", ParsedURL(Protocol.HTTP, "localhost", 5000, "/foo"), False),
        ("/foo", ParsedURL(Protocol.HTTP, "localhost", 80, "/foo"), False),
        (
            "/foo?bar=bar",
            ParsedURL(Protocol.HTTP, "localhost", 80, "/foo", "?bar=bar"),
            False,
        ),
        ("/", ParsedURL(Protocol.HTTP, "localhost", 80, "/", None), False),
        (":5000", None, True),
    ],
)
def test_parse_url(test_input: str, expected: ParsedURL, raises: bool):
    if raises:
        with pytest.raises(ClickException):
            UrlParser.parse(test_input)
    else:
        parsed_url = UrlParser.parse(test_input)
        assert parsed_url == expected
