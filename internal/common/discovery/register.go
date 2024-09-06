package discovery

import (
	"context"
	"log"
	"time"

	clientv3 "go.etcd.io/etcd/client/v3"
)

type Register struct {
	cli         *clientv3.Client
	key         string
	val         string
	ctx         context.Context
	keepAliveCh <-chan *clientv3.LeaseKeepAliveResponse
	leaseID     clientv3.LeaseID
}

func NewRegister(ctx context.Context, endpoints []string, key string, endportInfo *EndpointInfo, leaseTTL int64) *Register {
	cli, err := clientv3.New(clientv3.Config{
		Endpoints:   endpoints,
		DialTimeout: 5 * time.Second,
	})

	if err != nil {
		log.Fatal(err)
	}

	data, err := endportInfo.Marshal()
	if err != nil {
		log.Fatal(err)
	}

	r := &Register{
		cli: cli,
		key: key,
		val: string(data),
		ctx: ctx,
	}

	if err := r.putKeyLease(leaseTTL); err != nil {
		log.Fatal(err)
	}

	return r
}

func (r *Register) putKeyLease(lease int64) error {
	resp, err := r.cli.Grant(r.ctx, lease)
	if err != nil {
		return err
	}

	_, err = r.cli.Put(r.ctx, r.key, r.val, clientv3.WithLease(resp.ID))
	if err != nil {
		return err
	}

	leaseRespCh, err := r.cli.KeepAlive(r.ctx, resp.ID)
	if err != nil {
		return err
	}

	r.leaseID = resp.ID
	r.keepAliveCh = leaseRespCh

	return nil
}
