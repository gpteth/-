<template>
  <div>
    <h1>Login</h1>
    <form @submit.prevent="login">
      <div>
        <label for="email">Email:</label>
        <input type="email" id="email" v-model="userData.email" required>
      </div>
      <div>
        <label for="password">Password:</label>
        <input type="password" id="password" v-model="userData.password" required>
      </div>
      <button type="submit">Login</button>
    </form>
  </div>
</template>

<script>
export default {
  data() {
    return {
      userData: {
        email: '',
        password: '',
      },
    };
  },
  methods: {
    login() {
      // Send a POST request to your backend API for user authentication
      const apiUrl = '/api/login';
      fetch(apiUrl, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify(this.userData),
      })
        .then((response) => response.json())
        .then((data) => {
          // Store user authentication token in localStorage or a state management solution
          localStorage.setItem('token', data.token);

          // Redirect to a protected route
          this.$router.push({ name: 'dashboard' });
        })
        .catch((error) => {
          console.error('Error authenticating user:', error);
        });
    },
  },
};
</script>
