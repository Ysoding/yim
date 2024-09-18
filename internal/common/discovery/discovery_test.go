package discovery

import (
	"context"
	"fmt"
	"log"
	"testing"
	"time"
)

func TestDiscovery(t *testing.T) {
	ctx := context.Background()
	endpoints := []string{"localhost:2379"}

	ser, err := NewDiscovery(ctx, endpoints)
	if err != nil {
		log.Fatalln(err)
	}

	defer ser.Close()

	set := func(key, value string) {
		fmt.Println(fmt.Sprintf("set %s=%s", key, value))
	}
	del := func(key, value string) {
		fmt.Println(fmt.Sprintf("del %s=%s", key, value))
	}

	err = ser.Watch("/web/", set, del)
	if err != nil {
		log.Fatalln(err)
	}

	err = ser.Watch("/gRPC/", set, del)
	if err != nil {
		log.Fatalln(err)
	}

	for {
		select {
		case <-time.Tick(10 * time.Second):
		}
	}
}
