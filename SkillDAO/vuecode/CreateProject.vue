<template>
  <div>
    <h1>Create a New Project</h1>
    <form @submit.prevent="createProject">
      <div>
        <label for="title">Title:</label>
        <input type="text" id="title" v-model="projectData.title" required>
      </div>
      <div>
        <label for="description">Description:</label>
        <textarea id="description" v-model="projectData.description" required></textarea>
      </div>
      <button type="submit">Create Project</button>
    </form>
  </div>
</template>

<script>
export default {
  data() {
    return {
      projectData: {
        title: '',
        description: '',
      },
    };
  },
  methods: {
    createProject() {
      // Send a POST request to your backend API to create a new project
      const apiUrl = '/api/projects';
      fetch(apiUrl, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify(this.projectData),
      })
        .then((response) => response.json())
        .then(() => {
          this.$router.push({ name: 'home' }); // Redirect to the home page after project creation
        })
        .catch((error) => {
          console.error('Error creating project:', error);
        });
    },
  },
};
</script>
