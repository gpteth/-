# CryptoKingdom MMORPG

CryptoKingdom is a blockchain-based Massively Multiplayer Online Role-Playing Game (MMORPG) where players can explore virtual worlds, own land, build kingdoms, engage in battles, and trade while earning cryptocurrency.

## Project Structure

The project consists of three main components:

1. **Ethereum Smart Contracts (Rust)**
   - `crypto_kingdom.sol`: Solidity contract for land ownership and currency management.

2. **Node.js Back-end**
   - `server.js`: Main server file to handle HTTP requests and interact with the Ethereum blockchain.
   - `routes.js`: Define routes for handling game actions (e.g., buying land, trading).
   - `ethereum.js`: A module to interact with the Ethereum blockchain using web3.js or ethers.js.
   - `game_logic.js`: Implement game logic, such as battles, exploration, and in-game events.

3. **Front-end (Three.js)**
   - `index.html`: HTML file for the game's interface.
   - `main.js`: JavaScript file to initialize the Three.js scene and handle user interactions.
   - `player.js`: Define player character and controls.
   - `world.js`: Create and manage the virtual game world.
   - `networking.js`: Handle communication with the server for real-time updates.

## Setup

1. **Ethereum Smart Contracts (Rust)**
   - Deploy the `crypto_kingdom.sol` contract on the Ethereum blockchain.
   - Configure your server to interact with the deployed contract.

2. **Node.js Back-end**
   - Install Node.js if not already installed.
   - Run `npm install` to install dependencies.
   - Set up your Ethereum node connection in `ethereum.js`.
   - Start the server with `node server.js`.

3. **Front-end (Three.js)**
   - Host the front-end files on a web server.
   - Configure the server endpoint in `networking.js` to match your back-end server.

## Usage

- Visit the front-end website to play CryptoKingdom.
- Explore virtual worlds, buy land, build kingdoms, trade with other players, and engage in battles.
- Your in-game actions are recorded on the Ethereum blockchain.

## Contributing

Feel free to contribute to this project by creating issues, suggesting improvements, or submitting pull requests.

## License

This project is licensed under the [MIT License](LICENSE).

## Acknowledgments

Thanks to the open-source community for providing tools and libraries used in this project.
