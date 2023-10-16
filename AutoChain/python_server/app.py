from flask import Flask, request, jsonify

app = Flask(__name__)

# Mock database to store vehicle information (replace with a real database)
vehicles_db = [
    {"id": 1, "make": "Toyota", "model": "Camry", "year": 2020, "price": 25000, "owner": None},
    {"id": 2, "make": "Honda", "model": "Civic", "year": 2022, "price": 22000, "owner": None},
    {"id": 3, "make": "Ford", "model": "Escape", "year": 2021, "price": 28000, "owner": None},
]

# Mock Rust smart contract connection (replace with actual connection)
class RustSmartContract:
    def __init__(self):
        pass

    def buy_vehicle(self, vehicle_id):
        # Simulate a vehicle purchase in Rust smart contract (replace with actual contract interaction)
        pass

rust_contract = RustSmartContract()

@app.route('/list_vehicles', methods=['GET'])
def list_vehicles():
    # Return a list of available vehicles
    return jsonify({"vehicles": vehicles_db})

@app.route('/buy_vehicle', methods=['POST'])
def buy_vehicle():
    data = request.json
    vehicle_id = data.get("vehicle_id")
    
    # Find the vehicle in the database (replace with a real database query)
    vehicle = next((v for v in vehicles_db if v["id"] == vehicle_id), None)

    if vehicle:
        if vehicle["owner"] is None:
            # Simulate a purchase in the Rust smart contract
            rust_contract.buy_vehicle(vehicle_id)
            
            # Update the vehicle's owner in the database (simulating ownership transfer)
            vehicle["owner"] = data.get("buyer_name")
            
            return jsonify({"message": f"Vehicle {vehicle_id} purchased successfully."})
        else:
            return jsonify({"error": "Vehicle already sold."}), 400
    else:
        return jsonify({"error": "Vehicle not found."}), 404

@app.route('/add_vehicle', methods=['POST'])
def add_vehicle():
    data = request.json
    vehicle_id = len(vehicles_db) + 1
    new_vehicle = {
        "id": vehicle_id,
        "make": data.get("make"),
        "model": data.get("model"),
        "year": data.get("year"),
        "price": data.get("price
