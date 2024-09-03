from typing import List

import pytest
from click import ClickException

from purl.constants import DEFAULT_HEADERS
from purl.items_parser import ItemsParser, RequestItems


@pytest.mark.parametrize(
    "test_input,expected,raises",
    [
        (None, RequestItems(DEFAULT_HEADERS, {}), False),
        (
            ["Cache-Control:no-cache", "key=value"],
            RequestItems(
                DEFAULT_HEADERS | {"Cache-Control": "no-cache"}, {"key": "value"}
            ),
            False,
        ),
        (["somethingelse"], None, True),
    ],
)
def test_parse_items(
    test_input: List[str] | None, expected: RequestItems, raises: bool
):
    if raises:
        with pytest.raises(ClickException):
            ItemsParser.parse(test_input)
    else:
        parsed_items = ItemsParser.parse(test_input)
        assert parsed_items == expected
