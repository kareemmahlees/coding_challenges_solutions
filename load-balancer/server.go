package main

import (
	"net/http"
)

type Status string

const (
	HEALTHY Status = "healthy"
	DOWN    Status = "down"
)

type Server struct {
	url    string
	status Status
}

// NewServer does an initial health check on the server
// and sets the server status accordingly.
func NewServer(url string) *Server {
	server := &Server{url, HEALTHY}
	server.status = server.HealthCheck()

	return server
}

// HealthCheck issues a GET request to the server
// and insures that response status code is 200.
func (s *Server) HealthCheck() Status {
	resp, err := http.Get(s.url)
	if err != nil || resp.StatusCode != http.StatusOK {
		return DOWN
	}
	return HEALTHY
}
