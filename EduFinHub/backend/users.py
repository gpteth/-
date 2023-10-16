@app.route('/api/user/profile')
@login_required
def get_user_profile():
    user = load_user(session['user_id'])
    return jsonify({'user_id': user.id, 'email': user.email})
