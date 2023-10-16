# Pirate creation and management using a simplified in-memory database
pirates = {}

@app.route('/create_pirate', methods=['POST'])
@login_required
def create_pirate():
    if current_user.id not in pirates:
        pirates[current_user.id] = {'health': 100, 'level': 1, 'inventory': []}
        return jsonify({'message': 'Pirate created successfully'})
    return jsonify({'message': 'You already have a pirate'}), 400

@app.route('/get_pirate')
@login_required
def get_pirate():
    pirate = pirates.get(current_user.id)
    if pirate:
        return jsonify({'pirate': pirate})
    return jsonify({'message': 'You do not have a pirate yet'}), 404
