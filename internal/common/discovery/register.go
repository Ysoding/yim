package discovery

import (
	"context"
	"time"

	clientv3 "go.etcd.io/etcd/client/v3"
	"go.uber.org/zap"
)

type Register struct {
	cli         *clientv3.Client
	key         string
	val         string
	ctx         context.Context
	keepAliveCh <-chan *clientv3.LeaseKeepAliveResponse
	leaseID     clientv3.LeaseID
}

func NewRegister(ctx context.Context, endpoints []string, key string, endportInfo *EndpointInfo, leaseTTL int64) (*Register, error) {
	cli, err := clientv3.New(clientv3.Config{
		Endpoints:   endpoints,
		DialTimeout: 5 * time.Second,
	})

	if err != nil {
		return nil, err
	}

	data, err := endportInfo.Marshal()
	if err != nil {
		return nil, err
	}

	r := &Register{
		cli: cli,
		key: key,
		val: string(data),
		ctx: ctx,
	}

	if err := r.putKeyLease(leaseTTL); err != nil {
		return nil, err
	}

	return r, nil
}

// putKeyLease 设置租约
func (r *Register) putKeyLease(leaseTTL int64) error {
	resp, err := r.cli.Grant(r.ctx, leaseTTL)
	if err != nil {
		return err
	}

	// key 绑定到 lease，生命周期一致
	_, err = r.cli.Put(r.ctx, r.key, r.val, clientv3.WithLease(resp.ID))
	if err != nil {
		return err
	}

	// keepalive lease
	leaseRespCh, err := r.cli.KeepAlive(r.ctx, resp.ID)
	if err != nil {
		return err
	}

	r.leaseID = resp.ID
	r.keepAliveCh = leaseRespCh

	return nil
}

func (r *Register) UpdateValue(val *EndpointInfo) error {
	value, err := val.Marshal()
	if err != nil {
		return err
	}

	_, err = r.cli.Put(r.ctx, r.key, string(value), clientv3.WithLease(r.leaseID))
	if err != nil {
		return err
	}

	zap.L().Sugar().Infow("register UpdateValue", "leaseId", r.leaseID, "key", r.key, "val", r.val)
	return nil
}

func (r *Register) ListenLeaseKeepAliveResponseCh() {
	for resp := range r.keepAliveCh {
		zap.L().Sugar().Infow("lease success", "leaseId", r.leaseID, "key", r.key, "val", r.val, resp)
	}
	zap.L().Sugar().Infow("lease failed", "leaseId", r.leaseID, "key", r.key, "val", r.val)
}

func (r *Register) Close() error {
	if _, err := r.cli.Revoke(context.Background(), r.leaseID); err != nil {
		return err
	}

	zap.L().Sugar().Infow("lease close", "leaseId", r.leaseID, "key", r.key, "val", r.val)
	return r.cli.Close()
}
