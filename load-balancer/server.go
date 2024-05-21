package main

type Server struct {
	url string
}

func NewServer(url string) *Server {
	return &Server{url}
}
