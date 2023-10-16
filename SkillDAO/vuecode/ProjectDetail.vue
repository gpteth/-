<template>
  <div>
    <h1>{{ project.title }}</h1>
    <p>{{ project.description }}</p>
  </div>
</template>

<script>
export default {
  data() {
    return {
      project: null,
    };
  },
  mounted() {
    this.fetchProjectDetails();
  },
  methods: {
    fetchProjectDetails() {
      // Fetch project details from the backend API using the project ID from the route parameter
      const projectId = this.$route.params.id;
      const apiUrl = `/api/projects/${projectId}`;
      fetch(apiUrl)
        .then((response) => response.json())
        .then((data) => {
          this.project = data;
        })
        .catch((error) => {
          console.error('Error fetching project details:', error);
        });
    },
  },
};
</script>
