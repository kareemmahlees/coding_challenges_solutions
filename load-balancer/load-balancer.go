package main

import (
	"fmt"
	"io"
	"log"
	"net/http"
	"os"
	"time"
)

// LoadBalancer keeps track of healthy and down servers
// and forwards requets to healthy servers accordingly.
type LoadBalancer struct {
	healthyServers []*Server
	downServers    []*Server
	// idx is used to keep track of the current round-about.
	idx int
	// done is used to stop the ticker on exit.
	done <-chan os.Signal
}

// NewLoadBalancer reads arguments (server urls) passed to the program
// and creates a list of them returning an instance of LoadBalancer.
func NewLoadBalancer(done <-chan os.Signal) *LoadBalancer {
	healtyServers := []*Server{}
	downServers := []*Server{}

	for _, arg := range os.Args[1:] {
		server := NewServer(arg)

		switch server.status {
		case DOWN:
			downServers = append(downServers, server)
			break
		case HEALTHY:
			healtyServers = append(healtyServers, server)
		}
	}

	return &LoadBalancer{healthyServers: healtyServers, downServers: downServers, idx: 0, done: done}
}

// ListenAndServe spwans a health checker in a goroutine
// and spins up a new http server.
func (lb *LoadBalancer) ListenAndServe() {
	go lb.SpwanHealthChecker()

	http.HandleFunc("/", func(w http.ResponseWriter, r *http.Request) {
		server := lb.GetCurrentServer()
		if server == nil {
			w.WriteHeader(http.StatusInternalServerError)
			w.Write([]byte("NO healthy servers"))
			return
		}
		resp, err := http.Get(server.url)
		if err != nil {
			w.Write([]byte(fmt.Sprintf("Something went wrong: %v", err)))
		}
		defer resp.Body.Close()

		byteData, err := io.ReadAll(resp.Body)
		if err != nil {
			w.Write([]byte(fmt.Sprintf("Something went wrong: %v", err)))
		}

		w.Write([]byte(byteData))
	})

	go func() {
		log.Default().Println("Load balancer listening")
		if err := http.ListenAndServe(":80", nil); err != nil {
			log.Fatal(err)
			os.Exit(1)
		}
	}()

	for {
		select {
		case <-lb.done:
			os.Exit(0)
		}
	}

}

// getCurrentServer returns the next server in the
// list of healthy server in a round-about manner.
func (lb *LoadBalancer) GetCurrentServer() *Server {
	if len(lb.healthyServers) == 0 {
		return nil
	}

	if lb.idx == len(lb.healthyServers) {
		lb.idx = 0
	}

	curServer := lb.healthyServers[lb.idx]
	lb.idx++

	return curServer
}

func (lb *LoadBalancer) SpwanHealthChecker() {
	ticker := time.NewTicker(time.Duration(*healthCheckFreq) * time.Second)

	for {
		select {
		case <-lb.done:
			ticker.Stop()
			return
		case <-ticker.C:
			go lb.checkupOnDownServers()
			go lb.checkupOnHealthyServers()
		}
	}
}

// checkupOnDownServers periodically checks on down servers
// if any had returned alive.
func (lb *LoadBalancer) checkupOnDownServers() {
	newDownServers := []*Server{}
	for _, server := range lb.downServers {
		status := server.HealthCheck()
		switch status {
		case HEALTHY:
			lb.healthyServers = append(lb.healthyServers, server)
			break
		case DOWN:
			newDownServers = append(newDownServers, server)
		}
	}
	lb.downServers = newDownServers
}

// checkupOnHealthyServers periodically checks on healthy servers
// if any went down.
func (lb *LoadBalancer) checkupOnHealthyServers() {
	newHealthyServers := []*Server{}
	for _, server := range lb.healthyServers {
		status := server.HealthCheck()
		switch status {
		case DOWN:
			lb.downServers = append(lb.downServers, server)
			break
		case HEALTHY:
			newHealthyServers = append(newHealthyServers, server)
		}
	}
	lb.healthyServers = newHealthyServers
}
