package main

import (
	"flag"
	"os"
	"os/signal"
	"syscall"
)

var (
	healthCheckFreq = flag.Int("t", 5, "specifiy how frequent to health check the servers")
	maxConn         = flag.Int("c", 10, "specifiy max connections to be handled by a server")
)

func init() {
	flag.Parse()
}

func main() {
	done := make(chan os.Signal)
	signal.Notify(done, os.Interrupt, syscall.SIGTERM, syscall.SIGINT)

	loadBalancer := NewLoadBalancer(done)

	loadBalancer.ListenAndServe()
}
