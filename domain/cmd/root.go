package cmd

import "github.com/spf13/cobra"

var rootCmd = &cobra.Command{
	Use:   "rcauth",
	Short: "redcardinal authentication service",
}

func Execute() {
	cobra.CheckErr(rootCmd.Execute())
}
