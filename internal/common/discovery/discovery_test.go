package discovery

import (
	"context"
	"fmt"
	"testing"
	"time"
)

func TestServiceDiscovery(t *testing.T) {
	ctx := context.Background()
	endpoints := []string{"localhost:2379"}

	ser := NewDiscovery(ctx, endpoints)
	defer ser.Close()

	set := func(key, value string) {
		fmt.Println(fmt.Sprintf("set %s=%s", key, value))
	}
	del := func(key, value string) {
		fmt.Println(fmt.Sprintf("del %s=%s", key, value))
	}

	err := ser.Watch("/web/", set, del)
	if err != nil {
		panic(err)
	}

	err = ser.Watch("/gRPC/", set, del)
	if err != nil {
		panic(err)
	}

	time.Tick(10 * time.Second)
}
