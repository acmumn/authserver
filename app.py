from flask import Flask, request, jsonify, redirect
import acmjwt
import MySQLdb
from os import environ as env

app = Flask(__name__)
db = MySQLdb.connect(
	host=env["DB_HOST"], port=int(env["DB_PORT"]),
	user=env["DB_USER"], passwd=env["DB_PASSWD"],
	db=env["DB_DB"])
jwtsrv = acmjwt.AuthServer(env["AUTH_SECRET"])
LIFETIME = 60*60*24*265

@app.route('/')
def hello_world():
    return "This is the authserver microservice. You're in the wrong place, buddy"

@app.route('/validate/member', methods=['POST'])
def validate_member():
	try:
		token = jwtsrv.validate_member_token(request.get_data())
		return jsonify({"ok": True, "token":token})
	except BaseException as e:
		return jsonify({"ok": False, "err": "Unknown error."})

@app.route('/validate/service', methods=['POST'])
def validate_service():
	try:
		token = jwtsrv.validate_service_token(request.get_data())
		return jsonify({"ok": True, "token":token})
	except BaseException as e:
		return jsonify({"ok": False, "err": "Unknown error."})

@app.route("/magiclink/<uuid>")
def magiclink(uuid):
	cur = db.cursor()
	cur.execute("SELECT id, member_id FROM jwt_escrow WHERE secret=UNHEX(%s)",
			[uuid.replace("-", "")])
	link_res = cur.fetchall()
	if len(link_res)==1:
		print(link_res[0])
		record_id, userid = link_res[0]
		print(record_id, type(record_id))

		cur.execute("DELETE FROM jwt_escrow WHERE record_id=%i", [record_id])
		resp = redirect(request.query_string)

		token = jwtsrv.issue_member_token(userid, extra={"uuid":uuid})

		resp.set_cookie("jwtauth", token, domain=".acm.umn.edu", max_age=LIFETIME, secure=True)
		return resp
	else:
		return "Error"