package ui

import (
	"log"
	"strings"

	"github.com/Ysoding/yim/internal/cli/client/sdk"
	"github.com/charmbracelet/bubbles/textarea"
	"github.com/charmbracelet/bubbles/viewport"
	tea "github.com/charmbracelet/bubbletea"
	"github.com/charmbracelet/glamour"
	"github.com/charmbracelet/lipgloss"
)

type (
	errMsg  error
	imMsg   *sdk.Message
	sendMsg struct{}
)

type model struct {
	viewport    viewport.Model
	history     []string
	textarea    textarea.Model
	senderStyle lipgloss.Style
	err         error
	renderer    *glamour.TermRenderer
	chat        *sdk.Chat
}

var (
	senderStyle = lipgloss.NewStyle().Bold(true).Foreground(lipgloss.Color("5"))
	recvStyle   = lipgloss.NewStyle().Bold(true).Foreground(lipgloss.Color("6"))
)

func initialModel(chat *sdk.Chat) model {
	ta := textarea.New()
	ta.Placeholder = "Send a message..."
	ta.Focus()

	ta.Prompt = "┃ "
	ta.CharLimit = -1

	ta.SetWidth(50)
	ta.SetHeight(1)

	// Remove cursor line styling
	ta.FocusedStyle.CursorLine = lipgloss.NewStyle()
	ta.ShowLineNumbers = false

	vp := viewport.New(500, 20)
	ta.KeyMap.InsertNewline.SetEnabled(false)

	renderer, _ := glamour.NewTermRenderer(
		glamour.WithAutoStyle(),
		glamour.WithWordWrap(0),
	)

	return model{
		textarea:    ta,
		history:     []string{},
		viewport:    vp,
		senderStyle: lipgloss.NewStyle().Foreground(lipgloss.Color("5")),
		err:         nil,
		renderer:    renderer,
		chat:        chat,
	}
}

func (m model) Init() tea.Cmd {
	return textarea.Blink
}

func (m model) Update(msg tea.Msg) (tea.Model, tea.Cmd) {
	var (
		cmd  tea.Cmd
		cmds []tea.Cmd
	)

	m.textarea, cmd = m.textarea.Update(msg)
	cmds = append(cmds, cmd)

	m.viewport, cmd = m.viewport.Update(msg)
	cmds = append(cmds, cmd)

	switch msg := msg.(type) {
	case tea.KeyMsg:
		switch msg.Type {
		case tea.KeyCtrlC, tea.KeyEsc:
			return m, tea.Quit
		case tea.KeyEnter:
			input := strings.TrimSpace(m.textarea.Value())
			if input == "" {
				break
			}
			cmds = append(cmds, func() tea.Msg {
				m.chat.Send(&sdk.Message{
					Content: input,
					Name:    "xxx",
				})
				return sendMsg{}
			})

			prompt := senderStyle.Render("You: ")
			c, _ := m.renderer.Render(input)
			c = ensureTrailingNewLine(c)
			m.history = append(m.history, prompt+c)

			m.viewport.SetContent(strings.Join(m.history, ""))
			m.textarea.Reset()
			m.viewport.GotoBottom()
		}
	case sendMsg:
	case imMsg:
		prompt := recvStyle.Render(msg.Name + ": ")
		content, _ := m.renderer.Render(msg.Content)
		content = ensureTrailingNewLine(content)
		m.history = append(m.history, prompt+content)
		m.viewport.SetContent(strings.Join(m.history, ""))
		m.viewport.GotoBottom()
	case errMsg:
		m.err = msg
		return m, nil
	}
	return m, tea.Batch(cmds...)
}

func (m model) View() string {
	return lipgloss.JoinVertical(lipgloss.Left,
		m.viewport.View(),
		m.textarea.View(),
	)
}

func Run() {

	chat := sdk.NewChat("192.168.0.1:9999", "test1", "test1", "")
	p := tea.NewProgram(initialModel(chat))

	go func() {
		rcvCh := chat.Recv()
		for msg := range rcvCh {
			p.Send(imMsg(msg))
		}
	}()

	if _, err := p.Run(); err != nil {
		log.Fatal(err)
	}
}

func ensureTrailingNewLine(s string) string {
	if strings.HasSuffix(s, "\n") {
		return s
	}
	return s + "\n"
}
