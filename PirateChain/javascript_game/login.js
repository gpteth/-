// login.js
document.getElementById('loginButton').addEventListener('click', function () {
    const username = document.getElementById('username').value;
    const password = document.getElementById('password').value;
    
    // Send a POST request to the backend for user authentication
    fetch('/login', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
        },
        body: JSON.stringify({ username, password }),
    })
    .then(response => response.json())
    .then(data => {
        if (data.message === 'Login successful') {
            // Redirect to the game dashboard
            window.location.href = '/dashboard';
        } else {
            document.getElementById('errorMessage').textContent = 'Login failed. Please try again.';
        }
    })
    .catch(error => {
        console.error('Error:', error);
    });
});
