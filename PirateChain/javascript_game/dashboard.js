// dashboard.js
document.getElementById('createPirateButton').addEventListener('click', function () {
    // Send a POST request to create a new pirate
    fetch('/create_pirate', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
        },
    })
    .then(response => response.json())
    .then(data => {
        if (data.message === 'Pirate created successfully') {
            // Reload the dashboard to display pirate information
            window.location.reload();
        } else {
            console.error('Error:', data.error);
        }
    })
    .catch(error => {
        console.error('Error:', error);
    });
});

// Fetch and display pirate information (to be implemented)
function fetchPirateInfo() {
    // Send a GET request to retrieve pirate information
    fetch('/get_pirate')
    .then(response => response.json())
    .then(data => {
        // Update the HTML to display pirate information
        document.getElementById('pirateInfo').textContent = JSON.stringify(data.pirate, null, 4);
    })
    .catch(error => {
        console.error('Error:', error);
    });
}

// Call the fetchPirateInfo function to display pirate information on page load
fetchPirateInfo();
