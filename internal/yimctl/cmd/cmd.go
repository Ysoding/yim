package cmd

import (
	"io"
	"os"

	"github.com/Ysoding/yim/internal/yimctl/cmd/client"
	"github.com/spf13/cobra"
)

func NewDefaultYIMCtlCommand() *cobra.Command {
	return NewYIMCtlCommand(os.Stdin, os.Stdout, os.Stderr)
}

func NewYIMCtlCommand(in io.Reader, out, err io.Writer) *cobra.Command {
	cmds := &cobra.Command{
		Use:   "yimctl",
		Short: "yimctl!!!",
		Run: func(cmd *cobra.Command, args []string) {
			cmd.Help()
		},
	}

	cmds.AddCommand(
		client.NewClientCmd(),
	)
	return cmds
}
