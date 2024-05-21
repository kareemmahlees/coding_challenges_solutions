package main

import (
	"fmt"
	"os"
)

type LoadBalancer struct {
	servers []*Server
	idx     int
}

// NewLoadBalancer reads arguments (server urls) passed to the program
// and creates a list of them returning an instance of LoadBalancer.
func NewLoadBalancer() *LoadBalancer {
	servers := []*Server{}

	for _, arg := range os.Args[1:] {
		fmt.Printf("arg: %v\n", arg)
		servers = append(servers, NewServer(arg))
	}

	return &LoadBalancer{servers: servers, idx: 0}
}

// getCurrentServer returns the next server url in a
// round-about manner.
func (lb *LoadBalancer) GetCurrentServer() *Server {
	// if we are at the end, cycle back to the beginning
	if lb.idx == len(lb.servers) {
		lb.idx = 0
	}

	curServer := lb.servers[lb.idx]
	lb.idx++

	return curServer
}
