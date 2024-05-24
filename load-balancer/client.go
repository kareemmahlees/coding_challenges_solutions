package main

import (
	"net/http"
	"time"
)

type Client struct{}

// NewClient returns an http.Client, this is used in favour of
// http.Get() for example to take benefit from the keep-alive
// connection mechanism.
func NewClient() *http.Client {
	tr := &http.Transport{
		MaxIdleConnsPerHost: 1024,
		TLSHandshakeTimeout: 0 * time.Second,
	}
	client := &http.Client{Transport: tr}

	return client
}
