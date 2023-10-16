// Investment page logic
document.addEventListener('DOMContentLoaded', () => {
    const investmentForm = document.getElementById('investment-form');

    investmentForm.addEventListener('submit', async (e) => {
        e.preventDefault();
        const amount = document.getElementById('investment-amount').value;

        try {
            // Make an API request to create an investment
            await fetchApi(`${apiUrl}/users/${currentUser.id}/investments`, 'POST', { amount });

            // Handle successful investment and update UI
        } catch (error) {
            // Handle investment error
        }
    });
});
