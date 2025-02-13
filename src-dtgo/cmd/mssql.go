package cmd

import (
	"fmt"

	"github.com/spf13/cobra"
)

func init() {
	var host = ""
	var initDB = ""
	var username = ""
	var password = ""

	var mssqlCmd = &cobra.Command{
		Use:   "mssql",
		Short: "",
		Long:  "",
		Run: func(cmd *cobra.Command, args []string) {
			fmt.Println("host:", host)
			fmt.Println("init-db:", initDB)
			fmt.Println("username:", username)
			fmt.Println("password:", password)
		},
	}
	mssqlCmd.Flags().StringVarP(&host, "host", "", "", "")
	mssqlCmd.Flags().StringVarP(&initDB, "init-db", "", "", "")
	mssqlCmd.Flags().StringVarP(&username, "username", "", "", "")
	mssqlCmd.Flags().StringVarP(&password, "password", "", "", "")

	rootCmd.AddCommand(mssqlCmd)
}
