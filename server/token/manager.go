package token

import (
	"fmt"
	"time"

	jwt "github.com/dgrijalva/jwt-go"
	log "github.com/sirupsen/logrus"
)

type Manager struct {
	authSecret []byte
	maxAge     time.Duration
}

func NewManager(authSecret []byte, maxAge time.Duration) *Manager {
	return &Manager{authSecret: authSecret, maxAge: maxAge}
}

func (mgr *Manager) CheckToken(tok string) (Token, error) {
	token, err := jwt.Parse(tok, func(t *jwt.Token) (interface{}, error) {
		return mgr.authSecret, nil
	})
	if err != nil {
		return nil, err
	}

	claims := token.Claims.(jwt.MapClaims)

	now := time.Now().Unix()
	if iat, ok := claims["iat"].(int64); ok {
		if now < iat {
			return nil, ErrExpired
		}
	}
	if exp, ok := claims["exp"].(int64); ok {
		if exp < now {
			return nil, ErrExpired
		}
	}

	switch claims["type"] {
	case "member":
		return newMemberToken(claims["id"].(uint)), nil
	case "service":
		return newServiceToken(claims["name"].(string)), nil
	default:
		log.Panicf("Invalid token type %#v", claims["type"])
		panic("unreachable")
	}
}

func (mgr *Manager) IssueMemberToken(id uint) (string, error) {
	now := time.Now()
	return mgr.sign(jwt.MapClaims{
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
