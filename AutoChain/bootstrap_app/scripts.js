// JavaScript code for AutoChain frontend

// Mock vehicle data (replace with actual data from the backend)
const vehicles = [
    { id: 1, make: 'Toyota', model: 'Camry', year: 2020, price: 25000 },
    { id: 2, make: 'Honda', model: 'Civic', year: 2022, price: 22000 },
    { id: 3, make: 'Ford', model: 'Escape', year: 2021, price: 28000 },
];

// Function to display vehicle cards
function displayVehicleCards() {
    const vehicleList = document.getElementById('vehicleList');
    vehicleList.innerHTML = '';

    vehicles.forEach(vehicle => {
        const card = document.createElement('div');
        card.className = 'card mb-3';
        card.innerHTML = `
            <div class="card-body">
                <h5 class="card-title">${vehicle.make} ${vehicle.model}</h5>
                <p class="card-text">Year: ${vehicle.year}</p>
                <p class="card-text">Price: $${vehicle.price}</p>
                <button class="btn btn-primary" onclick="showVehicleDetails(${vehicle.id})">View Details</button>
            </div>
        `;

        vehicleList.appendChild(card);
    });
}

// Function to display vehicle details
function showVehicleDetails(vehicleId) {
    const vehicle = vehicles.find(v => v.id === vehicleId);
    const vehicleDetails = document.getElementById('vehicleDetails');
    vehicleDetails.innerHTML = '';

    if (vehicle) {
        vehicleDetails.innerHTML = `
            <h3>${vehicle.make} ${vehicle.model}</h3>
            <p>Year: ${vehicle.year}</p>
            <p>Price: $${vehicle.price}</p>
            <button class="btn btn-success" onclick="buyVehicle(${vehicle.id})">Buy Vehicle</button>
        `;
    }
}

// Function to simulate vehicle purchase (replace with backend functionality)
function buyVehicle(vehicleId) {
    alert(`You have purchased vehicle with ID: ${vehicleId}`);
    // Implement the purchase logic and update the backend here
}

// Initial page load
displayVehicleCards();
