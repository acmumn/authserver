package main

import "github.com/gin-gonic/gin"

// GetStatus is the handler for /status with the method GET.
func GetStatus(c *gin.Context) {
	c.Status(204)
}
