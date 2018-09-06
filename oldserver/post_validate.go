package main

import (
	"net/http"

	"github.com/acmumn/identity/server/db"
	"github.com/acmumn/identity/server/token"
	"github.com/gin-gonic/gin"
)

// PostValidate is the handler for /validate with the method POST.
func PostValidate(db *db.DB, toks *token.Manager) gin.HandlerFunc {
	return func(c *gin.Context) {
		_, err := c.Cookie("auth")
		if err != nil {
			c.String(http.StatusForbidden, "No authentication token present")
		}

		panic("TODO")
	}
}
