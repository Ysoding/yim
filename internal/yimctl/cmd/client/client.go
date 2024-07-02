package client

import (
	"github.com/Ysoding/yim/internal/cli/client/ui"
	"github.com/spf13/cobra"
)

func NewClientCmd() *cobra.Command {
	cmd := &cobra.Command{
		Use:   "client",
		Short: "create a command line client",
		Long:  "create a command line client",
		Run:   run,
	}

	return cmd
}

func run(cmd *cobra.Command, args []string) {
	ui.Run()
}
