import React, { useState, useEffect } from 'react';

function JobList() {
  const [jobs, setJobs] = useState([]);

  useEffect(() => {
    // Fetch jobs from your backend API endpoint, e.g., '/api/jobs'
    fetch('/api/jobs')
      .then((response) => response.json())
      .then((data) => setJobs(data))
      .catch((error) => console.error(error));
  }, []);

  return (
    <div>
      <h2>Job Listings</h2>
      <ul>
        {jobs.map((job) => (
          <li key={job.id}>
            <a href={`/job/${job.id}`}>
              <h3>{job.title}</h3>
            </a>
            <p>{job.description}</p>
          </li>
        ))}
      </ul>
    </div>
  );
}

export default JobList;
