# authserver

URL Structure
-------------

### POST `/validate`

Validate a JWT.
Submit with JWT as data.
If Valid, responds with 200 and the claims object as JSON. Otherwise, responds with 403.

### POST `/validate_service`

Validate a JWT for a service. Requires that the JWT was issued from the console and that it's `type` is that of a service token.
Submit with JWT as data.
If Valid, responds with 200 and the claims object as JSON. Otherwise, responds with 403.

### GET `/magiclink/<uuid>?<redir>`

If the UUID is present in the `jwt_escrow` table, it removes it, issues a JWT as a cookie for `.acm.umn.edu` and redirects to the redirect URL.