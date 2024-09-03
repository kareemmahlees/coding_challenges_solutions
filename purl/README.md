# Purl

Challenge link: https://codingchallenges.fyi/challenges/challenge-curl

Simply `Curl`, but more **P**owerfull.

Inspired by [httpie](https://httpie.io/cli).

Here is an example of a request being sent with `Curl` vs `Purl`:

**Curl**

```sh
curl -X POST http://eu.httpbin.org/post -d '{"key":"value"}' -H "Content-type:application/json" -H "Cache-Control:no-cache"
```

**Purl**

```sh
purl http://eu.httpbin.org/post POST Cache-Control:no-cache key=value
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
