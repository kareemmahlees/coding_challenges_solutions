# Load Balancer

challenge url : <https://codingchallenges.fyi/challenges/challenge-load-balancer/>

A Concurrent, Layer 7, Simple Load Balancer written in Golang.

It uses a static load balancing algorithm called `Round-Robin` where you cycle through the list
of servers handing the request to each one accordingly.

It also implements health checking mechanism that frequently updates
the internal list of healthy and down servers.

## How to use

```sh
$ go run . --help

-c int
    specifiy max connections to be handled by a server (default 10)
-t int
    specifiy how frequent to health check the servers (default 5)
```

```sh
# go run [list of servers' urls]
$ go run . http://localhost:8080 http://localhost:8081
```
