# Authentication using Flask-Login (you need to install the Flask-Login package)
from flask import Flask, request, jsonify, redirect, url_for
from flask_login import LoginManager, UserMixin, login_user, login_required, logout_user, current_user

app = Flask(__name__)
login_manager = LoginManager()
login_manager.init_app(app)

# User model (simplified)
class User(UserMixin):
    def __init__(self, id):
        self.id = id

# Simulated user database
users = {'user1': {'password': 'password1'}, 'user2': {'password': 'password2'}}

@login_manager.user_loader
def load_user(user_id):
    return User(user_id)

@app.route('/login', methods=['POST'])
def login():
    username = request.json['username']
    password = request.json['password']
    if users.get(username) and users[username]['password'] == password:
        user = User(username)
        login_user(user)
        return jsonify({'message': 'Login successful'})
    return jsonify({'message': 'Login failed'}), 401

@app.route('/logout')
@login_required
def logout():
    logout_user()
    return jsonify({'message': 'Logged out'})
