from flask import Flask, request, jsonify, redirect
import acmjwt
app = Flask(__name__)
secret = "oof"
LIFETIME = 60*60*24*265

@app.route('/')
def hello_world():
    return "This is the authserver microservice. You're in the wrong place, buddy"

@app.route('/validate', methods=['POST'])
def validate():
	try:
		token = acmjwt.validate_token(request.get_data(), secret, lifetime=LIFETIME)
		return jsonify({"ok": True, "x500": token["x500"], "token":token})
	except BaseException as e:
		print(e)
		return jsonify({"ok": False, "err": "Unknown error."})

@app.route("/magiclink/<uuid>")
def magiclink(uuid):
	resp = redirect(request.query_string)
	token = acmjwt.create_token({"x500":"goess006"}, secret)
	resp.set_cookie("jwtauth", token, domain=".acm.umn.edu", max_age=LIFETIME)
	return resp