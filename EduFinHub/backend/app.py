from flask import Flask, jsonify, request
from flask_cors import CORS
from models import Course, UserCourse  # Import database models
from sqlalchemy import create_engine
from sqlalchemy.orm import sessionmaker

app = Flask(__name__)
CORS(app)  # Enable Cross-Origin Resource Sharing (CORS)

# Configure the database connection
db_url = 'sqlite:///edufinhub.db'  # SQLite database (replace with your preferred DB)
engine = create_engine(db_url)
Session = sessionmaker(bind=engine)

# Sample route to test the backend
@app.route('/')
def hello_world():
    return 'Hello, EduFinHub Backend!'

# Route to fetch all courses
@app.route('/api/courses')
def get_courses():
    session = Session()
    courses = session.query(Course).all()
    course_list = [{'id': course.id, 'name': course.name, 'description': course.description, 'price': course.price} for course in courses]
    session.close()
    return jsonify(course_list)

# Route to enroll a user in a course
@app.route('/api/enroll', methods=['POST'])
def enroll_course():
    user_id = request.json.get('user_id')
    course_id = request.json.get('course_id')

    session = Session()
    user_course = UserCourse(user_id=user_id, course_id=course_id, status='Enrolled')
    session.add(user_course)
    session.commit()
    session.close()

    return jsonify({'message': 'Enrollment successful'})

if __name__ == '__main__':
    app.run(debug=True)
