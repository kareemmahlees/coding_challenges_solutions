package main

import (
	"io"
	"log"
	"net/http"
)

func main() {
	loadBalancer := NewLoadBalancer()

	http.HandleFunc("/", func(w http.ResponseWriter, r *http.Request) {
		server := loadBalancer.GetCurrentServer()
		resp, err := http.Get(server.url)
		if err != nil {
			log.Fatalf("Something went wrong: %v", err)
		}
		defer resp.Body.Close()

		byteData, err := io.ReadAll(resp.Body)
		if err != nil {
			log.Fatalf("Something went wrong: %v", err)
		}

		w.Write([]byte(byteData))
	})

	log.Default().Println("Load balancer listening")
	log.Fatal(http.ListenAndServe(":80", nil))
}
