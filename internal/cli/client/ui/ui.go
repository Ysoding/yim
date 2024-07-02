package ui

import (
	"fmt"
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
	recvMsg string
)

type model struct {
	viewport    viewport.Model
	messages    []string
	textarea    textarea.Model
	senderStyle lipgloss.Style
	err         error
	renderer    *glamour.TermRenderer
	input       string
	recv        string
	chat        *sdk.Chat
}

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

	vp := viewport.New(50, 5)
	ta.KeyMap.InsertNewline.SetEnabled(false)

	renderer, _ := glamour.NewTermRenderer(
		glamour.WithEnvironmentConfig(),
		glamour.WithWordWrap(0),
	)

	return model{
		textarea:    ta,
		messages:    []string{},
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
			fmt.Println(m.textarea.Value())
			return m, tea.Quit
		case tea.KeyEnter:
			input := strings.TrimSpace(m.textarea.Value())
			if input == "" {
				break
			}
			m.input = input
			m.viewport.SetContent(m.RenderConversation())
			m.textarea.Reset()
			m.viewport.GotoBottom()
		}

	case recvMsg:
		m.recv = string(msg)
		m.viewport.SetContent(m.RenderConversation())
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

var (
	senderStyle = lipgloss.NewStyle().Bold(true).Foreground(lipgloss.Color("5"))
)

func (m model) RenderConversation() string {
	var sb strings.Builder

	renderYou := func(content string) {
		sb.WriteString(senderStyle.Render("You: "))
		content, _ = m.renderer.Render(content)
		sb.WriteString(ensureTrailingNewLine(content))
	}

	renderRecv := func(content string) {
		sb.WriteString(senderStyle.Render("Recv: "))
		content, _ = m.renderer.Render(content)
		sb.WriteString(ensureTrailingNewLine(content))
	}

	if m.input != "" {
		renderYou(m.input)
	}

	if m.recv != "" {
		renderRecv(m.recv)
	}

	return sb.String()
}

func Run() {


	chat := sdk.NewChat("192.168.0.1:9999", "test1", "test1", "")
	p := tea.NewProgram(initialModel(chat))

	go doRecv()

	if _, err := p.Run(); err != nil {
		log.Fatal(err)
	}
}
func doRecv(chat*sdk.Chat) {
	for c := range 
}

func ensureTrailingNewLine(s string) string {
	if strings.HasSuffix(s, "\n") {
		return s
	}
	return s + "\n"
}
