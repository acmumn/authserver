package main

import (
	"github.com/acmumn/identity/server/db"
	"github.com/acmumn/identity/server/token"
	"github.com/gin-gonic/gin"
)

// PostValidate is the handler for /validate with the method POST.
func PostValidate(db *db.DB, toks *token.Manager) gin.HandlerFunc {
	return func(c *gin.Context) {
		// TODO
	}
}
