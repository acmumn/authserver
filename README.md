# authserver

URL Structure
-------------

### POST `/validate/member`

Validate a JWT for a member.
Submit with JWT as data.
Responds with a JSON object containing the fields `ok` (boolean) and `token` (an object containing the JWT's claims).

### POST `/validate/service`

Validate a JWT for a service.
Submit with JWT as data.
Responds with a JSON object containing the fields `ok` (boolean) and `token` (an object containing the JWT's claims).

### GET `/magiclink/<uuid>?<redir>`

If the UUID is present in the `jwt_escrow` table, it removes it, issues a JWT as a cookie for `.acm.umn.edu` and redirects to the redirect URL.