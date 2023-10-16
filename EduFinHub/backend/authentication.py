from flask import Flask, jsonify, request, session, redirect, url_for
from flask_cors import CORS
from flask_login import LoginManager, UserMixin, login_user, logout_user, login_required
from flask_wtf import FlaskForm
from wtforms import StringField, PasswordField, SubmitField
from wtforms.validators import DataRequired
from werkzeug.security import generate_password_hash, check_password_hash
from models import User, Course, UserCourse  # Import your models

app = Flask(__name__)
app.secret_key = 'your_secret_key_here'
CORS(app)

# Configure the database connection and create the session

# ...

login_manager = LoginManager()
login_manager.init_app(app)

class LoginForm(FlaskForm):
    email = StringField('Email', validators=[DataRequired()])
    password = PasswordField('Password', validators=[DataRequired()])
    submit = SubmitField('Login')

@login_manager.user_loader
def load_user(user_id):
    session = Session()
    return session.query(User).get(int(user_id))

# Route for user login
@app.route('/api/login', methods=['POST'])
def login():
    form = LoginForm()
    if form.validate_on_submit():
        email = form.email.data
        password = form.password.data

        session = Session()
        user = session.query(User).filter_by(email=email).first()

        if user and check_password_hash(user.password, password):
            login_user(user)
            return jsonify({'message': 'Login successful'})

    return jsonify({'message': 'Login failed'})

# Route for user registration
@app.route('/api/register', methods=['POST'])
def register():
    email = request.json.get('email')
    password = request.json.get('password')
    # Additional registration logic (e.g., validation)

    hashed_password = generate_password_hash(password)

    session = Session()
    new_user = User(email=email, password=hashed_password)
    session.add(new_user)
    session.commit()

    return jsonify({'message': 'Registration successful'})

# Route for user logout
@app.route('/api/logout')
@login_required
def logout():
    logout_user()
    return jsonify({'message': 'Logout successful'})

# Additional routes and features can be added here
