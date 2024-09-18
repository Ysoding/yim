package discovery

import (
	"context"
	"sync"
	"time"

	"go.etcd.io/etcd/api/v3/mvccpb"
	clientv3 "go.etcd.io/etcd/client/v3"
	"go.uber.org/zap"
)

type Discovery struct {
	cli *clientv3.Client
	mu  sync.Mutex
	ctx context.Context
}

func NewDiscovery(ctx context.Context, endpoints []string) (*Discovery, error) {
	cli, err := clientv3.New(clientv3.Config{
		Endpoints:   endpoints,
		DialTimeout: 5 * time.Second,
	})

	if err != nil {
		return nil, err
	}

	return &Discovery{cli: cli, ctx: ctx}, nil
}

func (d *Discovery) Watch(prefix string, set, del func(key, value string)) error {
	resp, err := d.cli.Get(d.ctx, prefix, clientv3.WithPrefix())
	if err != nil {
		return err
	}

	for _, ev := range resp.Kvs {
		set(string(ev.Key), string(ev.Value))
	}
	d.watcher(prefix, set, del)
	return nil
}

func (d *Discovery) watcher(prefix string, set, del func(key, value string)) {
	wcCh := d.cli.Watch(d.ctx, prefix, clientv3.WithPrefix())
	zap.L().Sugar().Infof("watching prefix:%s", prefix)

	for resp := range wcCh {
		for _, ev := range resp.Events {
			switch ev.Type {
			case mvccpb.PUT:
				set(string(ev.Kv.Key), string(ev.Kv.Value))
			case mvccpb.DELETE:
				del(string(ev.Kv.Key), string(ev.Kv.Value))
			}
		}
	}
}

func (d *Discovery) Close() error {
	return d.cli.Close()
}
