package client

import (
	"fmt"

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
	fmt.Print("command line client")
}
