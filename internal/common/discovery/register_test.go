package discovery

import (
	"context"
	"log"
	"testing"
	"time"
)

func TestRegister(t *testing.T) {
	ctx := context.Background()
	endpoints := []string{"localhost:2379"}

	r, err := NewRegister(ctx, endpoints, "/web/node1", &EndpointInfo{IP: "127.0.0.1", Port: "6969"}, 5)
	if err != nil {
		log.Fatalln(err)
	}

	go r.ListenLeaseKeepAliveResponseCh()
	select {
	case <-time.After(10 * time.Second):
		r.Close()
	}
}
