// frontend/src/components/NFT/NFTList.js
import React, { useEffect, useState } from 'react';
import { View, Text, FlatList, StyleSheet } from 'react-native';

function NFTList() {
  const [nfts, setNFTs] = useState([]);

  useEffect(() => {
    // Fetch the list of NFTs from your backend
    // Update the 'nfts' state with the fetched data
  }, []);

  return (
    <View style={styles.container}>
      <Text>NFT List</Text>
      <FlatList
        data={nfts}
        keyExtractor={(item) => item.id.toString()}
        renderItem={({ item }) => (
          <View style={styles.nftItem}>
            <Text>{item.name}</Text>
            <Text>{item.description}</Text>
            <Text>Owner: {item.owner}</Text>
          </View>
        )}
      />
    </View>
  );
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
    justifyContent: 'center',
    alignItems: 'center',
  },
  nftItem: {
    padding: 10,
    marginBottom: 10,
    borderWidth: 1,
    borderColor: '#ccc',
  },
});

export default NFTList;
