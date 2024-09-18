package domain

type Endpoint struct {
	IP          string       `json:"ip"`
	Port        string       `json:"port"`
	ActiveScore float64      `json:"-"`
	StaticScore float64      `json:"-"`
	Stat        *Stat        `json:"-"`
	window      *stateWindow `json:"-"`
}

func NewEndpoint(ip, port string) *Endpoint {
	ep := &Endpoint{
		IP:     ip,
		Port:   port,
		window: newStateWindow(),
		Stat:   &Stat{},
	}

	return ep
}

func (ep *Endpoint) UpdateStat(s *Stat) {
	ep.window.appendStat(s)
	ep.Stat = ep.window.getStat()
}

func (ep *Endpoint) UpdateScore() {
	if ep.Stat != nil {
		ep.ActiveScore = ep.Stat.CalculateActiveScore()
		ep.StaticScore = ep.Stat.CalculateStaticScore()
	}
}
