// Define global variables and constants
const apiUrl = 'https://api.educhain.com'; // Replace with your API endpoint
const token = 'YOUR_AUTH_TOKEN'; // Replace with your authentication token (if applicable)

// Function to fetch data from the API
async function fetchData(endpoint, options = {}) {
    try {
        const response = await fetch(`${apiUrl}${endpoint}`, {
            headers: {
                Authorization: `Bearer ${token}`,
                'Content-Type': 'application/json',
            },
            ...options,
        });

        if (!response.ok) {
            throw new Error(`Error: ${response.status} - ${response.statusText}`);
        }

        return await response.json();
    } catch (error) {
        console.error('Fetch Error:', error);
        // Handle error gracefully (e.g., show an error message to the user)
    }
}

// Function to load user data and display the dashboard
async function loadDashboard() {
    try {
        const userData = await fetchData('/user/profile');
        const certifications = await fetchData('/user/certifications');
        const courses = await fetchData('/user/courses');

        // Update the dashboard content
        displayUserData(userData);
        displayCertifications(certifications);
        displayCourses(courses);
    } catch (error) {
        console.error('Dashboard Load Error:', error);
        // Handle error gracefully (e.g., show an error message to the user)
    }
}

// Function to display user data in the dashboard
function displayUserData(userData) {
    const userNameElement = document.getElementById('user-name');
    userNameElement.textContent = userData.name;

    // Add more user data fields as needed
}

// Function to display user certifications
function displayCertifications(certifications) {
    const certificationsList = document.getElementById('certifications-list');
    certificationsList.innerHTML = ''; // Clear previous content

    for (const certification of certifications) {
        const certificationCard = createCertificationCard(certification);
        certificationsList.appendChild(certificationCard);
    }
}

// Function to create a certification card
function createCertificationCard(certification) {
    const card = document.createElement('div');
    card.classList.add('card');

    const certificationName = document.createElement('h3');
    certificationName.textContent = certification.name;
    card.appendChild(certificationName);

    // Add more certification details (e.g., date, issuer)

    return card;
}

// Function to display user courses
function displayCourses(courses) {
    const coursesList = document.getElementById('courses-list');
    coursesList.innerHTML = ''; // Clear previous content

    for (const course of courses) {
        const courseCard = createCourseCard(course);
        coursesList.appendChild(courseCard);
    }
}

// Function to create a course card
function createCourseCard(course) {
    const card = document.createElement('div');
    card.classList.add('card');

    const courseName = document.createElement('h3');
    courseName.textContent = course.name;
    card.appendChild(courseName);

    // Add more course details (e.g., description, instructor)

    return card;
}

// Event listener for the logout button
document.getElementById('logout-button').addEventListener('click', function () {
    // Handle user logout (e.g., clear session, redirect to login page)
    // Example: Implement a logout function
});

// Initialize the application when the page loads
document.addEventListener('DOMContentLoaded', function () {
    // Check if the user is authenticated (you would use a token-based system)
    const isAuthenticated = true; // Replace with your authentication logic

    if (isAuthenticated) {
        // Load the user dashboard
        loadDashboard();
    } else {
        // Display a login or registration form
        displayLoginForm();
    }
});
