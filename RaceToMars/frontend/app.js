// frontend/App.js
import React, { useState, useEffect } from 'react';
import { View, Text, Button, TextInput, StyleSheet, FlatList } from 'react-native';

export default function App() {
    const [user, setUser] = useState(null);
    const [spacecrafts, setSpacecrafts] = useState([]);
    const [newSpacecraftName, setNewSpacecraftName] = useState('');

    useEffect(() => {
        // Fetch user data from the backend and set it in the state
        // Example: fetch('/api/users/1').then(response => response.json()).then(data => setUser(data));

        // Fetch user's spacecraft data
        // Example: fetch('/api/spacecrafts').then(response => response.json()).then(data => setSpacecrafts(data));
    }, []);

    const handleBuildSpacecraft = () => {
        // Implement logic to build a spacecraft and make a POST request to the backend
        // Example:
        // fetch('/api/spacecrafts', {
        //     method: 'POST',
        //     headers: {
        //         'Content-Type': 'application/json',
        //     },
        //     body: JSON.stringify({ name: newSpacecraftName }),
        // })
        //     .then(response => response.json())
        //     .then(data => {
        //         setSpacecrafts([...spacecrafts, data]);
        //         setNewSpacecraftName('');
        //     });
    };

    const handleLaunchToMars = (spacecraftId) => {
        // Implement logic to send a spacecraft to Mars and make a POST request to the backend
        // Example:
        // fetch(`/api/spacecrafts/${spacecraftId}/launch`, {
        //     method: 'POST',
        // })
        //     .then(response => response.json())
        //     .then(data => {
        //         // Update the spacecraft status in the state
        //         const updatedSpacecrafts = spacecrafts.map(spacecraft => {
        //             if (spacecraft.id === spacecraftId) {
        //                 return { ...spacecraft, isOnMars: true };
        //             }
        //             return spacecraft;
        //         });
        //         setSpacecrafts(updatedSpacecrafts);
        //     });
    };

    return (
        <View style={styles.container}>
            {user ? (
                <Text>Welcome, {user.name}!</Text>
            ) : (
                <Text>Loading...</Text>
            )}

            <Text style={styles.heading}>Your Spacecrafts</Text>
            <TextInput
                style={styles.input}
                placeholder="Enter spacecraft name"
                value={newSpacecraftName}
                onChangeText={setNewSpacecraftName}
            />
            <Button
                title="Build Spacecraft"
                onPress={handleBuildSpacecraft}
            />

            <FlatList
                data={spacecrafts}
                keyExtractor={(item) => item.id.toString()}
                renderItem={({ item }) => (
                    <View style={styles.spacecraftItem}>
                        <Text>Name: {item.name}</Text>
                        <Text>Parts: {item.parts}</Text>
                        {item.isOnMars ? (
                            <Text>Status: On Mars</Text>
                        ) : (
                            <Button
                                title="Launch to Mars"
                                onPress={() => handleLaunchToMars(item.id)}
                            />
                        )}
                    </View>
                )}
            />
        </View>
    );
}

const styles = StyleSheet.create({
    container: {
        flex: 1,
        alignItems: 'center',
        justifyContent: 'center',
        padding: 16,
    },
    heading: {
        fontSize: 20,
        fontWeight: 'bold',
        marginTop: 20,
    },
    input: {
        borderWidth: 1,
        borderColor: 'gray',
        width: '80%',
        padding: 8,
        marginTop: 10,
    },
    spacecraftItem: {
        borderWidth: 1,
        borderColor: 'lightgray',
        padding: 16,
        marginVertical: 8,
        width: '80%',
        alignSelf: 'center',
    },
});
