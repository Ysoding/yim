package discovery

import (
	"encoding/json"
)

type EndpointInfo struct {
	IP       string                 `json:"ip"`
	Port     string                 `json:"port"`
	MetaData map[string]interface{} `json:"meta"`
}

func (e *EndpointInfo) Marshal() ([]byte, error) {
	data, err := json.Marshal(e)
	if err != nil {
		return nil, err
	}
	return data, nil
}

func (e *EndpointInfo) Unmarshal(data []byte) error {
	err := json.Unmarshal(data, e)
	if err != nil {
		return err
	}
	return nil
}
