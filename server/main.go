package main

import (
	"fmt"
	"log"
	"os"

	"github.com/acmumn/identity/server/db"
	"github.com/gin-gonic/gin"
	"github.com/joho/godotenv"
	"github.com/urfave/cli"
)

func main() {
	if err := godotenv.Load(); err != nil {
		log.Fatal("Error loading .env file")
	}

	// Gin appears not to pick up on its mode when the GIN_MODE env var is loaded via godotenv...
	if ginMode := os.Getenv("GIN_MODE"); ginMode != "" {
		gin.SetMode(ginMode)
	}

	app := cli.NewApp()
	app.Name = "identity-server"
	app.Usage = "The identity server"
	app.Flags = []cli.Flag{
		cli.StringFlag{
			Name:   "auth-secret",
			EnvVar: "AUTH_SECRET",
			Usage:  "The HS512 secret",
		},
		cli.StringFlag{
			Name:   "base-url",
			EnvVar: "BASE_URL",
			Usage:  "The base URL for magic links",
		},
		cli.StringFlag{
			Name:   "database-url",
			EnvVar: "DATABASE_URL",
			Usage:  "The MySQL database URL",
		},
		cli.StringFlag{
			Name:   "host",
			EnvVar: "HOST",
			Usage:  "The IP to bind to",
			Value:  "",
		},
		cli.UintFlag{
			Name:   "port",
			EnvVar: "PORT",
			Usage:  "The port to bind to",
			Value:  8000,
		},
		cli.StringFlag{
			Name:   "syslog-server",
			EnvVar: "SYSLOG_SERVER",
			Usage:  "The syslog server to send logs to",
			Value:  "",
		},
	}
	app.Action = run

	if err := app.Run(os.Args); err != nil {
		log.Fatal(err)
	}
}

func run(c *cli.Context) error {
	db, err := db.Connect(c.GlobalString("database-url"))
	if err != nil {
		return err
	}
	defer db.Close()

	r := gin.New()
	r.Use(gin.Logger())
	r.Use(gin.Recovery())

	return r.Run(fmt.Sprintf("%s:%d", c.GlobalString("host"), c.GlobalUint("port")))
}
