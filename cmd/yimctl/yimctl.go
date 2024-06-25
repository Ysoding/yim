package main

import (
	"os"

	"github.com/Ysoding/yim/internal/yimctl/cmd"
)

func main() {
	command := cmd.NewDefaultYIMCtlCommand()
	if err := command.Execute(); err != nil {
		os.Exit(1)
	}
}
