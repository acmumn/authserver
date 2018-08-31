package token

import "errors"

var (
	// The token expired.
	ErrExpired = errors.New("The auth token expired")
)
