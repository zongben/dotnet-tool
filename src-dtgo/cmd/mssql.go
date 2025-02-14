package cmd

import (
	"database/sql"
	"fmt"

	"github.com/spf13/cobra"
  _ "github.com/microsoft/go-mssqldb"
)

func init() {
  host := ""
  initDB := ""
  username := ""
  password := ""

	var mssqlCmd = &cobra.Command{
		Use:   "mssql",
		Short: "",
		Long:  "",
		Run: func(cmd *cobra.Command, args []string) {
			fmt.Println("host:", host)
			fmt.Println("init-db:", initDB)
			fmt.Println("username:", username)
			fmt.Println("password:", password)

			connect_str := fmt.Sprintf("server=%s;user id=%s;password=%s;database=%s", host, username, password, initDB)
      fmt.Println(connect_str)

      db, err := sql.Open("sqlserver", connect_str)
      if err != nil {
        fmt.Println(err)
        return
      }
      defer db.Close()

      rows, err := db.Query("SELECT TABLE_NAME FROM INFORMATION_SCHEMA.TABLES")
      if err != nil {
        fmt.Println(err)
        return
      }
      defer rows.Close()

      for rows.Next() {
        table_name := ""
        err := rows.Scan(&table_name)
        if err != nil {
          fmt.Println(err)
          return
        }
        fmt.Println(table_name)
      }
		},
	}
	mssqlCmd.Flags().StringVarP(&host, "host", "", "", "")
	mssqlCmd.Flags().StringVarP(&initDB, "init-db", "", "", "")
	mssqlCmd.Flags().StringVarP(&username, "username", "", "", "")
	mssqlCmd.Flags().StringVarP(&password, "password", "", "", "")

	rootCmd.AddCommand(mssqlCmd)
}
