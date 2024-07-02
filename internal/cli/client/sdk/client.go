package sdk

type MsgType string

const (
	TextMsg  MsgType = "text"
	VideoMsg MsgType = "video"
	ImageMsg MsgType = "image"
)

type Chat struct {
	Nickname  string
	UserID    string
	SessionID string
	conn      *connect
}

type Message struct {
	Type       MsgType
	Name       string
	FromUserID string
	ToUserID   string
	Content    string
	SessionID  string
}

type connect struct {
	serverAddr         string
	sendChan, recvChan chan *Message
}

func NewChat(serverAddr, nickname, userID, sessionID string) *Chat {
	return &Chat{
		conn:      newConnect(serverAddr),
		Nickname:  nickname,
		UserID:    userID,
		SessionID: serverAddr,
	}
}

func (c *Chat) Send(msg *Message) {
	c.conn.send(msg)
}

func (c *Chat) Recv() chan *Message {
	return c.conn.recv()
}

func (c *Chat) Close() {
	c.conn.close()
}

func newConnect(serverAddr string) *connect {
	return &connect{
		serverAddr: serverAddr,
		sendChan:   make(chan *Message),
		recvChan:   make(chan *Message),
	}
}

func (c *connect) send(msg *Message) {
	c.recvChan <- msg
	// c.sendChan <- msg
}

func (c *connect) recv() chan *Message {
	return c.recvChan
}

func (c *connect) close() {

}
