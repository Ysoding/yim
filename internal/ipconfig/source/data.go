package source

import (
	"context"

	"github.com/Ysoding/yim/internal/common/discovery"
	"go.uber.org/zap"
)

func Init() {
	eventChan = make(chan *Event, 100)
	ctx := context.Background()
	go dataHandler(ctx)
}

func dataHandler(ctx context.Context) {
	dis, err := discovery.NewDiscovery(ctx, []string{"localhost:2379"})
	if err != nil {
		panic(err)
	}

	defer dis.Close()

	setFn := func(key, value string) {
		ed := &discovery.EndpointInfo{}

		if err := ed.Unmarshal([]byte(value)); err == nil {
			if event, err := NewEvent(ed); err == nil {
				event.Type = AddNodeEvent
				eventChan <- event
			} else {
				zap.L().Sugar().Infow("dataHandler.setFn.NewEvent", "err", err.Error())
			}
		} else {
			zap.L().Sugar().Infow("dataHandler.setFn", "err", err.Error())
		}
	}

	delFn := func(key, value string) {
		var ed *discovery.EndpointInfo
		if err := ed.Unmarshal([]byte(value)); err == nil {
			if event, err := NewEvent(ed); err == nil {
				event.Type = DelNodeEvent
				eventChan <- event
			} else {
				zap.L().Sugar().Infow(" dataHandler.delFn.NewEvent", "err", err.Error())
			}
		} else {
			zap.L().Sugar().Infow("dataHandler.delFn", "err", err.Error())
		}
	}

	err = dis.Watch("/yim/ip_dispatcher", setFn, delFn)
	if err != nil {
		panic(err)
	}
}
