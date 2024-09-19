package main

import (
	"os"

	"github.com/Ysoding/yim/internal/yimctl/cmd"
	"go.uber.org/zap"
)

func init() {
	logger, err := zap.NewProduction()
	if err != nil {
		panic(err)
	}
	zap.ReplaceGlobals(logger)
}

func main() {
	command := cmd.NewDefaultYIMCtlCommand()
	if err := command.Execute(); err != nil {
		os.Exit(1)
	}
}
