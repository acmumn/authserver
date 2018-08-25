from authlib.specs.rfc7519 import JWT
import time

ALGORITHM = "HS512"

BASE_NAME = "acmjwtauth"
ISS_WEBLINK = BASE_NAME+".weblink"
ISS_CONSOLE = BASE_NAME+".console"

TYPE_MEMBER = "member"
TYPE_SERVICE = "service"

SERVICE_TOKEN = object()

class AuthServer:
	def __init__(self, secret):
		self.secret = secret

	def issue_raw_token(self, data, iss_mode, lifetime=60*60*24*365, nbf_time=None):
		iat_time = int(time.time())
		if nbf_time is None:
			nbf_time = iat_time
		payload = {
			"iss": iss_mode,
			"aud": "acm.*",
			"iat": iat_time,
			"nbf": nbf_time,
			"exp": nbf_time + lifetime,
			**data
		}
		return JWT().encode({"alg": ALGORITHM}, payload, self.secret).decode("ascii")

	def validate_token(self, data, allowed_sources=[ISS_WEBLINK, ISS_CONSOLE]):
		claims = JWT().decode(data, self.secret, claims_options={
			"iss": {
				"essential": True,
				"values": allowed_sources
			}
		})
		claims.validate()
		return dict(claims)

	def issue_member_token(self, userid, source=ISS_WEBLINK, extra={}):
		return self.issue_raw_token({"type":TYPE_MEMBER, "id":userid, **extra}, source)

if __name__=="__main__":
	print("Issuing a service token...")
	jwtsrv = AuthServer(input("Secret: "))
	print(jwtsrv.issue_raw_token({"type":TYPE_SERVICE, "name":input("Service Name: ")}, ISS_CONSOLE))