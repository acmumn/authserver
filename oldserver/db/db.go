package db

import (
	"database/sql"

	_ "github.com/go-sql-driver/mysql"
	uuid "github.com/satori/go.uuid"
)

type DB struct {
	conn *sql.DB

	getMemberIdAndEmailFromX500 *sql.Stmt
	getMemberIdFromUUID         *sql.Stmt
	deleteUUID                  *sql.Stmt
	insertUUID                  *sql.Stmt
}

// Connect opens a connection to the database.
func Connect(addr string) (*DB, error) {
	conn, err := sql.Open("mysql", addr)
	if err != nil {
		return nil, err
	}

	getMemberIdAndEmailFromX500, err := conn.Prepare("")
	if err != nil {
		return nil, err
	}

	getMemberIdFromUUID, err := conn.Prepare("")
	if err != nil {
		return nil, err
	}

	deleteUUID, err := conn.Prepare("")
	if err != nil {
		return nil, err
	}

	insertUUID, err := conn.Prepare("INSERT INTO ")
	if err != nil {
		return nil, err
	}

	return &DB{conn, getMemberIdAndEmailFromX500, getMemberIdFromUUID, deleteUUID, insertUUID}, nil
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

// GetMemberIDAndEmail returns the id and email corresponding to the given X.500.
func (db *DB) GetMemberIDAndEmailFromX500(x500 string) (uint, string, error) {
	panic("TODO")
}

// GetAndRemoveLoginUUID checks for a login UUID. If it exists, it deletes it and returns the
// corresponding member ID. Otherwise, it returns sql.ErrNoRows.
func GetAndRemoveLoginUUID(uuid uuid.UUID) (uint, error) {
	panic("TODO")
}

// NewLoginUUID creates a new login UUID for the member with the given ID, registers it with the
// database, and returns it.
func (db *DB) NewLoginUUID(member uint) (uuid.UUID, error) {
	panic("TODO")
}
