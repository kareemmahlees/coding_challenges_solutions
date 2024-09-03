from enum import StrEnum


class RequestMethod(StrEnum):
    """
    Enum represeting supported http methods by `purl`.
    """

    GET = "GET"
    DELETE = "DELETE"
    POST = "POST"
    PUT = "PUT"


class Protocol(StrEnum):
    """
    Supported protocols by `perl`
    """

    HTTP = "HTTP"
    HTTPS = "HTTPs"
    UNSUPPORTED = "uNSUPported"


class ContentType(StrEnum):
    """
    Utility enum representing major `Content-type` headers.
    """

    Json = "application/json"
    Text = "text/plain"
    Html = "text/html"
    Form = "application/x-www-form-urlencoded"
