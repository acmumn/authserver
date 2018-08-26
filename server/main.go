package main

import (
	"fmt"
	"html/template"
	"log/syslog"
	"net/url"
	"os"
	"time"

	"github.com/acmumn/identity/server/db"
	"github.com/acmumn/identity/server/token"
	"github.com/acmumn/mailer/go-client"
	"github.com/gin-gonic/contrib/ginrus"
	"github.com/gin-gonic/gin"
	"github.com/joho/godotenv"
	log "github.com/sirupsen/logrus"
	syslogHook "github.com/sirupsen/logrus/hooks/syslog"
	"github.com/urfave/cli"
)

func main() {
	if err := godotenv.Load(); err != nil {
		log.Println("Error loading .env file")
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
			Usage:  "The HS512 secret.",
		},
		cli.StringFlag{
			Name:   "auth-token",
			EnvVar: "AUTH_TOKEN",
			Usage:  "This service's authentication token. If not provided, one will be generated.",
		},
		cli.StringFlag{
			Name:   "base-url",
			EnvVar: "BASE_URL",
			Usage:  "The base URL for magic links.",
		},
		cli.StringFlag{
			Name:   "database-url",
			EnvVar: "DATABASE_URL",
			Usage:  "The MySQL database URL.",
		},
		cli.StringFlag{
			Name:   "host",
			EnvVar: "HOST",
			Usage:  "The IP to bind to.",
			Value:  "",
		},
		cli.StringFlag{
			Name:   "mailer-server",
			EnvVar: "MAILER_SERVER",
			Usage:  "The URL of the mailer server to use..",
		},
		cli.UintFlag{
			Name:   "port",
			EnvVar: "PORT",
			Usage:  "The port to bind to.",
			Value:  8000,
		},
		cli.StringFlag{
			Name:   "syslog-server",
			EnvVar: "SYSLOG_SERVER",
			Usage:  "The syslog server to send logs to.",
			Value:  "",
		},
	}
	app.Action = run

	if err := app.Run(os.Args); err != nil {
		log.Fatal(err)
	}
}

func run(c *cli.Context) error {
	if addr := c.GlobalString("syslog-server"); addr != "" {
		hook, err := syslogHook.NewSyslogHook("tcp", addr, syslog.LOG_NOTICE|syslog.LOG_DAEMON,
			"identity")
		if err != nil {
			return err
		}
		log.AddHook(hook)
	}

	templates, err := loadTemplates(c.GlobalString("base-url"))
	if err != nil {
		return err
	}

	db, err := db.Connect(c.GlobalString("database-url"))
	if err != nil {
		return err
	}
	defer db.Close()

	toks := token.NewManager([]byte(c.GlobalString("auth-secret")), 365*24*time.Hour)

	authToken := c.GlobalString("auth-token")
	if authToken == "" {
		authToken, err = toks.IssueServiceToken("Identity Service", false)
		if err != nil {
			return err
		}
	}

	mailerServerURL, err := url.Parse(c.GlobalString("mailer-server"))
	if err != nil {
		return err
	}

	mailer := mailer.New(mailerServerURL, authToken)

	r := gin.New()
	r.Use(ginrus.Ginrus(log.StandardLogger(), time.RFC3339, false))
	r.Use(gin.Recovery())
	r.SetHTMLTemplate(templates)

	r.GET("/", GetIndex)
	r.POST("/", PostIndex(db, mailer))
	r.GET("/login/:uuid", GetLogin(db, toks))
	r.GET("/status", GetStatus)
	r.POST("/validate", PostValidate(db, toks))

	r.GET("/main.css", Static("/assets/main.css", "text/css"))

	addr := fmt.Sprintf("%s:%d", c.GlobalString("host"), c.GlobalUint("port"))
	log.Infof("Starting on %s...", addr)
	return r.Run(addr)
}

func loadTemplates(baseURL string) (*template.Template, error) {
	base, err := url.Parse(baseURL)
	if err != nil {
		return nil, err
	}

	t := template.New("").Funcs(template.FuncMap{
		"RelativeURL": func(rhs string) (string, error) {
			rhsURL, err := url.Parse(rhs)
			if err != nil {
				return "", err
			}

			return base.ResolveReference(rhsURL).String(), nil
		},
	})
	for _, name := range []string{"error", "get-index", "post-index"} {
		data := string(Assets.Files[fmt.Sprintf("/assets/%s.html", name)].Data)
		template.Must(t.New(name).Parse(data))
	}

	return t, nil
}
