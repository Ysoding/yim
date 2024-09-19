package domain

import (
	"sort"
	"sync"

	"github.com/Ysoding/yim/internal/ipconfig/source"
)

type Dispatcher struct {
	candidateMap map[string]*Endpoint
	mu           *sync.RWMutex
}

var dp *Dispatcher

func Init() {
	dp = &Dispatcher{
		candidateMap: map[string]*Endpoint{},
		mu:           &sync.RWMutex{},
	}
	go func() {
		for event := range source.EventChan() {
			switch event.Type {
			case source.AddNodeEvent:
				dp.addNode(event)
			case source.DelNodeEvent:
				dp.delNode(event)
			}
		}
	}()
}

func Dispatch(ctx *IPConfigContext) []*Endpoint {
	eps := dp.getCandidateEndpoints()

	for _, ep := range eps {
		ep.UpdateScore()
	}

	sort.Slice(eps, func(i, j int) bool {
		cmp := eps[i].ActiveScore - eps[j].ActiveScore
		if cmp > 0 {
			return true
		} else if cmp < 0 {
			return false
		} else {
			return eps[i].StaticScore > eps[j].StaticScore
		}
	})

	return eps
}

func (dp *Dispatcher) getCandidateEndpoints() []*Endpoint {
	dp.mu.RLock()
	defer dp.mu.RUnlock()

	res := make([]*Endpoint, 0, len(dp.candidateMap))
	for _, ep := range dp.candidateMap {
		res = append(res, ep)
	}
	return res
}

func (dp *Dispatcher) delNode(event *source.Event) {
	dp.mu.Lock()
	defer dp.mu.Unlock()
	delete(dp.candidateMap, event.Key())
}

func (dp *Dispatcher) addNode(event *source.Event) {
	dp.mu.Lock()
	defer dp.mu.Unlock()

	ep := NewEndpoint(event.IP, event.Port)
	ep.UpdateStat(&Stat{
		ConnectNum:   event.ConnectNum,
		MessageBytes: event.MessageBytes,
	})

	dp.candidateMap[event.Key()] = ep
}
