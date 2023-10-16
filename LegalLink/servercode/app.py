from flask import Flask, render_template, request, jsonify, redirect, url_for, abort

app = Flask(__name__)

# Sample data structure to store contracts (replace with a database in a real product)
contracts = []

@app.route('/')
def index():
    return render_template('index.html')

@app.route('/create_contract', methods=['GET', 'POST'])
def create_contract():
    if request.method == 'POST':
        # Extract contract data from the form
        title = request.form.get('contract-title')
        description = request.form.get('contract-description')
        # Extract more contract data fields as needed

        # Create a new contract and store it (replace with database storage)
        new_contract = {
            'title': title,
            'description': description,
            # Add more contract data fields as needed
        }
        contracts.append(new_contract)

        # Redirect to the contract details page
        return redirect(url_for('contract_details', contract_id=len(contracts) - 1))

    return render_template('create_contract.html')

@app.route('/contracts', methods=['GET'])
def list_contracts():
    # Return a list of contract titles and their IDs
    contract_list = [{'id': idx, 'title': contract['title']} for idx, contract in enumerate(contracts)]
    return render_template('list_contracts.html', contracts=contract_list)

@app.route('/contracts/<int:contract_id>', methods=['GET'])
def contract_details(contract_id):
    if 0 <= contract_id < len(contracts):
        # Retrieve the contract by its ID
        contract = contracts[contract_id]
        return render_template('contract_details.html', contract=contract)
    else:
        return 'Contract not found', 404

@app.route('/contracts/<int:contract_id>/edit', methods=['GET', 'POST'])
def edit_contract(contract_id):
    if request.method == 'POST':
        if 0 <= contract_id < len(contracts):
            # Update contract data based on the form submission
            contracts[contract_id]['title'] = request.form.get('contract-title')
            contracts[contract_id]['description'] = request.form.get('contract-description')
            # Update more contract data fields as needed

            # Redirect to the contract details page after editing
            return redirect(url_for('contract_details', contract_id=contract_id))
    elif request.method == 'GET':
        if 0 <= contract_id < len(contracts):
            # Display the contract editing form
            return render_template('edit_contract.html', contract=contracts[contract_id])
    
    abort(404)

@app.route('/contracts/<int:contract_id>/delete', methods=['POST'])
def delete_contract(contract_id):
    if 0 <= contract_id < len(contracts):
        # Delete the contract by its ID
        del contracts[contract_id]

        # Redirect to the list of contracts after deletion
        return redirect(url_for('list_contracts'))
    
    abort(404)

if __name__ == '__main__':
    app.run(debug=True)

