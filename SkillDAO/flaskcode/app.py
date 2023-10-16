from flask import Flask, request, jsonify
from flask_sqlalchemy import SQLAlchemy
from flask_migrate import Migrate
from datetime import datetime

app = Flask(__name__)

# Replace with your actual database URI
app.config['SQLALCHEMY_DATABASE_URI'] = 'sqlite:///skilldao.db'
db = SQLAlchemy(app)
migrate = Migrate(app, db)

# Define the Project model
class Project(db.Model):
    id = db.Column(db.Integer, primary_key=True)
    title = db.Column(db.String(100), nullable=False)
    description = db.Column(db.Text, nullable=False)
    created_at = db.Column(db.DateTime, default=datetime.utcnow)

# Create the database tables
db.create_all()

# API endpoint to create a new project
@app.route('/api/projects', methods=['POST'])
def create_project():
    data = request.get_json()
    title = data.get('title')
    description = data.get('description')

    if not title or not description:
        return jsonify({'message': 'Title and description are required'}), 400

    project = Project(title=title, description=description)
    db.session.add(project)
    db.session.commit()

    return jsonify({'message': 'Project created successfully'}), 201

# API endpoint to list all projects
@app.route('/api/projects', methods=['GET'])
def get_projects():
    projects = Project.query.all()
    project_list = [{'id': project.id, 'title': project.title, 'description': project.description} for project in projects]
    return jsonify(project_list)

if __name__ == '__main__':
    app.run(debug=True)
