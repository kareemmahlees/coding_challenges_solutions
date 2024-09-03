# Purl

Challenge link: https://codingchallenges.fyi/challenges/challenge-curl

Simply `Curl`, but more **P**owerfull.

Inspired by [httpie](https://httpie.io/cli).

Here is an example of a request being sent with `Curl` vs `Purl`:

**Curl**

```sh
curl -X POST http://localhost:5000/post -d '{"key":"value"}' -H "Content-type:application/json"
```

**Purl**

```sh
purl :5000/post POST key=value
```

## How to use

Purl uses [uv](https://docs.astral.sh/uv/) as it's package manager, after your have cloned the repo, install the dependencies:

```sh
uv sync
```

And now simply run:

```sh
python main.py --help
```

Or run the tests:

```sh
pytest -vv
```

## Overview

### Usage

```sh
purl [OPTIONS] URL [METHOD] [ITEMS]...
```

### Localhost friendly

Purl knows your needs as a developer, and tries to make it easier for your. Purl has shortcuts for localhost that makes it more convenient to work locally.

For example, `:3000` would expand to `http://localhost:3000`. If the port is omitted, then port `80` is assumed.

```sh
purl /foo

Host: http://localhost:80/foo
```

```sh
purl :3000/bar

Host: http://localhost:3000/bar
```

### Methods

Methods are passed as an optional argument after the url, all requests default to the `GET` method if no method is provided.

### Headers

You don't need to pass an additional flag or wrap your headers in string, **you just pass them**:

```sh
purl https://eu.httpbin.org/get Cache-Control:no-cache
```

**Default headers**:
| Key | Value |
|---|---|
| `Content-type` | `application/json` |
| `Connection` | `close`|
|`Accept` | `*/*`|

> [!NOTE]
> All the default headers are overridable if you pass them manually.

### JSON

Purl has first class support for json data, the default `Content-type` header is `application/json`, and you can pass json key value pairs with ease:

```sh
purl https://eu.httpbin.org/post POST key=value
```

### Form

If you want to switch the data your are sending from `Json` to `Form` data, you can just pass the `-f` flag and purl will take care of the headers for you.

### Flags

- **--offline**

  Don't send the request, only construct it and show what _will be sent_.

- **--verbose**

  Show more output about what's happening.
