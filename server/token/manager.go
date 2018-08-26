package token

import (
	"fmt"
	"time"

	jwt "github.com/dgrijalva/jwt-go"
)

type Manager struct {
	authSecret []byte
	maxAge     time.Duration
}

func NewManager(authSecret []byte, maxAge time.Duration) *Manager {
	return &Manager{authSecret: authSecret, maxAge: maxAge}
}

func (mgr *Manager) IssueMemberToken(id uint) (string, error) {
	now := time.Now()
	return mgr.sign(map[string]interface{}{
		"iat":  now.Unix(),
		"exp":  now.Add(mgr.maxAge).Unix(),
		"type": "member",
		"id":   fmt.Sprint(id),
	})
}

func (mgr *Manager) IssueServiceToken(name string, expires bool) (string, error) {
	data := map[string]interface{}{
		"type": "service",
		"name": name,
	}
	if expires {
		now := time.Now()
		data["iat"] = now.Unix()
		data["exp"] = now.Add(mgr.maxAge).Unix()
	}
	return mgr.sign(data)
}

func (mgr *Manager) sign(data map[string]interface{}) (string, error) {
	tok := jwt.NewWithClaims(jwt.SigningMethodHS512, jwt.MapClaims(data))
	return tok.SignedString(mgr.authSecret)
}
