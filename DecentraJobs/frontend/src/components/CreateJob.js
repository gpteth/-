import React, { useState } from 'react';

function CreateJob() {
  const [formData, setFormData] = useState({
    title: '',
    description: '',
    salary: '',
  });

  const handleChange = (e) => {
    const { name, value } = e.target;
    setFormData({ ...formData, [name]: value });
  };

  const handleSubmit = (e) => {
    e.preventDefault();
    // Send a POST request to your backend API to create a new job listing
    fetch('/api/jobs', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify(formData),
    })
      .then((response) => response.json())
      .then((data) => {
        // Handle success or error
        console.log('Job created:', data);
      })
      .catch((error) => console.error(error));
  };

  return (
    <div>
      <h2>Create a New Job Listing</h2>
      <form onSubmit={handleSubmit}>
        <div>
          <label>Title:</label>
          <input
            type="text"
            name="title"
            value={formData.title}
            onChange={handleChange}
          />
        </div>
        <div>
          <label>Description:</label>
          <textarea
            name="description"
            value={formData.description}
            onChange={handleChange}
          ></textarea>
        </div>
        <div>
          <label>Salary:</label>
          <input
            type="number"
            name="salary"
            value={formData.salary}
            onChange={handleChange}
          />
        </div>
        <div>
          <button type="submit">Create Job</button>
        </div>
      </form>
    </div>
  );
}

export default CreateJob;
