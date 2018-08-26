package main

import (
	"net/http"

	"github.com/gin-gonic/gin"
)

// Static serves a file from the compiled-in assets.
func Static(name, contentType string) gin.HandlerFunc {
	data := Assets.Files[name].Data
	return func(c *gin.Context) {
		c.Data(http.StatusOK, contentType, data)
	}
}
