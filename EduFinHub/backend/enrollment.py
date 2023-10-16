# Route for enrolling in a course
@app.route('/api/enroll', methods=['POST'])
@login_required
def enroll_course():
    user_id = session['user_id']
    course_id = request.json.get('course_id')

    session = Session()
    user_course = UserCourse(user_id=user_id, course_id=course_id, status='Enrolled')
    session.add(user_course)
    session.commit()

    return jsonify({'message': 'Enrollment successful'})

# Route for fetching a list of courses enrolled by a user
@app.route('/api/user/courses')
@login_required
def get_user_courses():
    user_id = session['user_id']
    session = Session()
    user_courses = session.query(UserCourse).filter_by(user_id=user_id).all()
    course_list = [{'id': course.id, 'name': course.course.name, 'status': course.status} for course in user_courses]
    return jsonify(course_list)

