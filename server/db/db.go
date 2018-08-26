package db

import (
	"database/sql"

	_ "github.com/go-sql-driver/mysql"
)

type DB struct {
	conn *sql.DB
}

// Connect opens a connection to the database.
func Connect(addr string) (*DB, error) {
	conn, err := sql.Open("mysql", addr)
	if err != nil {
		return nil, err
	}

	return &DB{conn}, nil
}

// Close closes the connection to the database.
func (db *DB) Close() error {
	var err error

	err = db.conn.Close()
	if err != nil {
		return err
	}

	return nil
}
