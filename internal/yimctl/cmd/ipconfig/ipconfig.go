package ipconfig

import (
	"github.com/Ysoding/yim/internal/ipconfig"
	"github.com/spf13/cobra"
)

func NewClientCmd() *cobra.Command {
	cmd := &cobra.Command{
		Use:   "ipconfig",
		Short: "ip config server",
		Run:   run,
	}

	return cmd
}

func run(cmd *cobra.Command, args []string) {
	ipconfig.Run()
}
