package main

import (
	"net/http"

	"github.com/gin-gonic/gin"
)

// GetIndex is the handler for / with the method GET.
func GetIndex(c *gin.Context) {
	redirect := c.Query("redirect")
	c.HTML(http.StatusOK, "get-index", gin.H{"Redirect": redirect})
}
