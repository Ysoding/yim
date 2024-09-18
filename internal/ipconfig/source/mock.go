package source

import (
	"context"
	"fmt"
	"time"

	"github.com/Ysoding/yim/internal/common/discovery"
	"golang.org/x/exp/rand"
)

func StartMock() {
	ctx := context.Background()
	go testRegister(ctx, "6969", "node1")
	go testRegister(ctx, "6968", "node2")
	go testRegister(ctx, "6967", "node3")
}

func testRegister(ctx context.Context, port, node string) {
	ed := discovery.EndpointInfo{
		IP:   "127.0.0.1",
		Port: port,
		MetaData: map[string]interface{}{
			"connect_num":   float64(rand.Int63n(696969)),
			"message_bytes": float64(rand.Int63n(69696969)),
		},
	}

	r, err := discovery.NewRegister(ctx, []string{"localhost:2379"}, fmt.Sprintf("/yim/ip_dispatcher/%s", node), &ed, time.Now().Unix())
	if err != nil {
		panic(err)
	}

	go r.ListenLeaseKeepAliveResponseCh()

	for {
		ed = discovery.EndpointInfo{
			IP:   "127.0.0.1",
			Port: port,
			MetaData: map[string]interface{}{
				"connect_num":   float64(rand.Int63n(696969)),
				"message_bytes": float64(rand.Int63n(69696969)),
			},
		}
		r.UpdateValue(&ed)
		time.Sleep(1 * time.Second)
	}

}
