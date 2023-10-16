// Token creation page logic
document.addEventListener('DOMContentLoaded', () => {
    const createTokenForm = document.getElementById('create-token-form');

    createTokenForm.addEventListener('submit', async (e) => {
        e.preventDefault();

        const name = document.getElementById('token-name').value;
        
        try {
            // Make an API request to create a token
            await fetchApi(`${apiUrl}/influencers/${currentUser.id}/tokens`, 'POST', { name });

            // Handle successful token creation and update UI
        } catch (error) {
            // Handle token creation error
        }
    });
});
