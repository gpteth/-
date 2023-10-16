import React, { useState, useEffect } from 'react';

function ArtworkList() {
  const [artworks, setArtworks] = useState([]);

  // Fetch artworks from your backend API and update the state
  useEffect(() => {
    fetch('/api/artworks')
      .then((response) => response.json())
      .then((data) => setArtworks(data));
  }, []);

  return (
    <div>
      <h2>Artworks</h2>
      <ul>
        {artworks.map((artwork) => (
          <li key={artwork.id}>{artwork.title}</li>
        ))}
      </ul>
    </div>
  );
}

export default ArtworkList;

