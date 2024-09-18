package source

import (
	"errors"
	"fmt"

	"github.com/Ysoding/yim/internal/common/discovery"
)

type EventType string

var eventChan chan *Event

func EventChan() <-chan *Event {
	return eventChan
}

const (
	AddNodeEvent EventType = "addNode"
	DelNodeEvent EventType = "delNode"
)

type Event struct {
	IP           string
	Port         string
	Type         EventType
	ConnectNum   float64
	MessageBytes float64
}

func NewEvent(ed *discovery.EndpointInfo) (*Event, error) {
	if ed.MetaData == nil {
		return nil, errors.New("endpoint metadata should not nil")
	}

	e := &Event{
		IP:   ed.IP,
		Port: ed.Port,
		Type: AddNodeEvent,
	}

	if data, ok := ed.MetaData["connect_num"]; ok {
		if v, ok := data.(float64); !ok {
			return nil, errors.New("endpoint metadata connect_num cannot be converted to float64")
		} else {
			e.ConnectNum = v
		}
	}

	if data, ok := ed.MetaData["message_bytes"]; ok {
		if v, ok := data.(float64); !ok {
			return nil, errors.New("endpoint metadata message_bytes cannot be converted to float64")
		} else {
			e.MessageBytes = v
		}
	}

	return e, nil
}

func (e *Event) Key() string {
	return fmt.Sprintf("%s:%s", e.IP, e.Port)
}
