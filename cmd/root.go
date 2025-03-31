package cmd

import (
	"context"

	"github.com/redcardinal-io/auth/internal/conf"
	"github.com/sirupsen/logrus"
	"github.com/spf13/cobra"
)

var (
	configFile = ""
	watchDir   = ""
)

var rootCmd = cobra.Command{
	Use: "rcauth",
	Run: func(cmd *cobra.Command, args []string) {},
}

func RootCommand() *cobra.Command {
	rootCmd.PersistentFlags().StringVarP(&configFile, "config", "c", "", "base configuration file to load")
	rootCmd.PersistentFlags().StringVarP(&watchDir, "config-dir", "d", "", "directory containing a sorted list of config files to watch for changes")
	return &rootCmd
}

func loadGlobalConfig(ctx context.Context) *conf.GlobalConfiguration {
	if ctx == nil {
		panic("context must not be nil")
	}

	config, err := conf.LoadGlobal(configFile)
	if err != nil {
		logrus.Fatalf("failed to load configuration: %+v", err)
	}

	return config
}

func execWithConfigAndArgs(cmd *cobra.Command, fn func(config *conf.GlobalConfiguration, args []string), args []string) {
	fn(loadGlobalConfig(cmd.Context()), args)
}
