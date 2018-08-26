Identity Server
===============

Configuration
-------------

Configured via either environment variables or a `.env` file. The following environment variables are used:

```
# Required
AUTH_SECRET="hunter2" # HS512 secret
BASE_URL="https://auth.acm.umn.edu" # Base URL for magic links
DATABASE_URL="mysql://root:password@localhost/acm" # MySQL database URL
GIN_MODE="release" # Set to enable Gin's release mode
MAILER_SERVER="https://mail.acm.umn.edu" # The URL of the mailer server to use

# Optional
AUTH_TOKEN="..." # This service's authentication token; will be generated if not provided
HOST="" # IP to bind to
PORT=8000 # Port to serve unsub links and template examples on
SYSLOG_SERVER="" # If non-empty, the syslog server to send logs to
```

URL Structure
-------------

### GET `/`

Serves a web form for a user to request to be mailed a "magic link." If the `redirect` query parameter is present, the magic link will redirect to it.

### POST `/`

Causes the user to be mailed a "magic link."

A request `Content-Type` of `application/x-www-form-urlencoded` is required. Expects two body values, `redirect` and `x500`. `redirect` is the URL the magic link should redirect to, and `x500` is the X.500 ID of the user to be mailed.

### GET `/login/<uuid>`

If the UUID is valid, invalidates it and issues an authentication token via the `auth` cookie and responds with status `303` with the `Location` set to either `acm.umn.edu` or the `redirect` query parameter, if present.

If the UUID is not valid, responds with status `404` and a web page asking the user to try again.

### GET `/status`

Always responds with an HTTP 204.

### POST `/validate`

Requires a service authentication token.

Validates the authentication token given as the (`text/plain`) body. If the token is valid, responds with status `200` and an object like one of the following:

```json
{ "iat": 1535259626
, "exp": 1566795651
, "type": "member"
, "id": 12
, "name": "Example Jones"
, "x500": "jones132"
, "card": "01234567890123456"
, "email": "jones132@umn.edu"
, "admin": false
, "paid": true
}

{ "iat": 1535259881
, "exp": 1566795879
, "type": "service"
, "name": "devnull-as-a-service"
}
```

If the token is not valid, responds with status `400` and an object like the following:

```json
{ "type": "expired" }
{ "type": "invalid" }
```

If the service authentication token provided is invalid, responds with status 403.
