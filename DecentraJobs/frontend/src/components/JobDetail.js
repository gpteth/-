import React, { useState, useEffect } from 'react';
import { useParams } from 'react-router-dom';

function JobDetail() {
  const { id } = useParams();
  const [job, setJob] = useState(null);

  useEffect(() => {
    // Fetch job details from your backend API, e.g., `/api/jobs/${id}`
    fetch(`/api/jobs/${id}`)
      .then((response) => response.json())
      .then((data) => setJob(data))
      .catch((error) => console.error(error));
  }, [id]);

  if (!job) {
    return <div>Loading...</div>;
  }

  return (
    <div>
      <h2>{job.title}</h2>
      <p>{job.description}</p>
      <p>Salary: {job.salary}</p>
    </div>
  );
}

export default JobDetail;
