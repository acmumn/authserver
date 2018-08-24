from authlib.specs.rfc7519 import JWT
import time

ALGORITHM = "HS512"
ISS_VALUE = "acmumn_jwtauth"
SUB_VALUE = "acmumn_identity"
AUD_VALUE = "acmumn_services"

def create_token(data, secret, lifetime=60*60*24*365, nbf_time=None):
	iat_time = int(time.time())
	if nbf_time is None:
		nbf_time = iat_time
	payload = {
		"iss": ISS_VALUE,
		"sub": SUB_VALUE,
		"aud": AUD_VALUE,
		"iat": iat_time,
		"nbf": nbf_time,
		"exp": nbf_time + lifetime,
		**data
	}
	return JWT().encode({"alg": ALGORITHM}, payload, secret).decode("ascii")

def validate_token(data, secret):
	claims = JWT().decode(data, secret, claims_options={
		"iss": {
			"essential": True,
			"value": ISS_VALUE
		},
		"sub": {
			"essential": True,
			"value": SUB_VALUE
		},
		"aud": {
			"essential": True,
			"value": AUD_VALUE
		}
	})
	claims.validate()
	return dict(claims)
