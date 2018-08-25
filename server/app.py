from flask import Flask, request, jsonify, redirect
import acmjwt
import MySQLdb
from os import environ as env

app = Flask(__name__)
jwtsrv = acmjwt.AuthServer(env["AUTH_SECRET"])
LIFETIME = 60*60*24*265

def getdb():
	return MySQLdb.connect(
		host=env["DB_HOST"], port=int(env["DB_PORT"]),
		user=env["DB_USER"], passwd=env["DB_PASSWD"],
		db=env["DB_DB"])

@app.route('/')
def hello_world():
    return "This is the authserver microservice. You're in the wrong place, buddy"

@app.route('/validate', methods=['POST'])
def validate():
	try:
		token = jwtsrv.validate_token(request.get_data())
		return jsonify(token)
	except BaseException as e:
		return "Invalid JWT", 403

@app.route('/validate_service', methods=['POST'])
def validate_service():
	try:
		token = jwtsrv.validate_token(request.get_data(), [acmjwt.ISS_CONSOLE])
		if token["type"] != acmjwt.TYPE_SERVICE:
			raise ValueError("Not a Service token")
		return jsonify(token)
	except BaseException as e:
		return "Invalid JWT", 403

@app.route("/magiclink/<uuid>")
def magiclink(uuid):
	db = getdb()
	cur = db.cursor()
	cur.execute("SELECT id, member_id FROM jwt_escrow WHERE secret=UNHEX(%s)",
			[uuid.replace("-", "")])
	link_res = cur.fetchall()
	if len(link_res)==1:
		record_id, userid = link_res[0]

		cur.execute("DELETE FROM jwt_escrow WHERE id=%s", [record_id])
		db.commit()
		db.close()

		redir=request.query_string.decode("ascii")
		if redir=="":
			redir="https://acm.umn.edu"
		resp = redirect(redir)

		token = jwtsrv.issue_member_token(userid, extra={"uuid":uuid})

		resp.set_cookie("jwtauth", token, domain=".acm.umn.edu", max_age=LIFETIME, secure=True)
		return resp
	else:
		return "Sorry, bad magic link"