// frontend/src/components/NFT/CreateNFT.js
import React, { useState } from 'react';
import { View, Text, TextInput, Button, StyleSheet } from 'react-native';

function CreateNFT() {
  const [name, setName] = useState('');
  const [description, setDescription] = useState('');
  const [image, setImage] = useState('');

  const handleCreateNFT = () => {
    // Implement NFT creation logic here (e.g., API call)
  };

  return (
    <View style={styles.container}>
      <Text>Create NFT</Text>
      <TextInput
        placeholder="Name"
        value={name}
        onChangeText={(text) => setName(text)}
        style={styles.input}
      />
      <TextInput
        placeholder="Description"
        value={description}
        onChangeText={(text) => setDescription(text)}
        style={styles.input}
      />
      <TextInput
        placeholder="Image URL"
        value={image}
        onChangeText={(text) => setImage(text)}
        style={styles.input}
      />
      <Button title="Create NFT" onPress={handleCreateNFT} />
    </View>
  );
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
    justifyContent: 'center',
    alignItems: 'center',
  },
  input: {
    width: 300,
    marginBottom: 10,
    padding: 10,
    borderWidth: 1,
    borderColor: '#ccc',
  },
});

export default CreateNFT;
