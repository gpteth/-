const Wallet = require('@solana/wallet');
const UserModel = require('../models/UserModel');

// User login
const login = async (req, res) => {
  try {
    const { username, password } = req.body;
    // Assume you have a UserModel schema for storing user data in MongoDB
    const user = await UserModel.findOne({ username, password });

    if (user) {
      // In a real application, generate and return a JWT token for authentication
      res.status(200).json({ success: true, message: 'Login successful', user });
    } else {
      res.status(401).json({ success: false, error: 'Invalid credentials' });
    }
  } catch (error) {
    console.error(error);
    res.status(500).json({ success: false, error: 'Login failed' });
  }
};

// Create Solana wallet for a user
const createWallet = async (req, res) => {
  try {
    const { privateKey } = req.body;

    // Initialize a Solana wallet with a private key
    const wallet = new Wallet(privateKey);

    // Store the user's wallet information in your database
    // This is a simplified example; in a real application, you'd handle wallet storage securely
    const user = new UserModel({
      username: 'example_username', // Replace with actual username
      password: 'example_password', // Replace with actual password
      ethereumAddress: wallet.ethereumAddress,
      privateKey: wallet.privateKey,
    });

    await user.save();

    res.status(200).json({ success: true, message: 'Wallet created successfully', wallet });
  } catch (error) {
    console.error(error);
    res.status(500).json({ success: false, error: 'Wallet creation failed' });
  }
};

// Transfer Solana tokens from one wallet to another
const transferTokens = async (req, res) => {
  try {
    const { senderPrivateKey, recipientPublicKey, amount } = req.body;

    // Initialize sender and recipient wallets
    const senderWallet = new Wallet(senderPrivateKey);
    const senderPublicKey = senderWallet.publicKey;

    // Create a Solana transaction to transfer tokens
    const transaction = new Transaction().add(
      SystemProgram.transfer({
        fromPubkey: senderPublicKey,
        toPubkey: new PublicKey(recipientPublicKey),
        lamports: amount, // Amount in lamports (1 SOL = 10^9 lamports)
      })
    );

    // Sign and send the transaction
    const signature = await sendAndConfirmTransaction(connection, transaction, [senderWallet]);

    res.status(200).json({ success: true, message: 'Token transfer successful', transactionSignature: signature });
  } catch (error) {
    console.error(error);
    res.status(500).json({ success: false, error: 'Token transfer failed' });
  }
};

module.exports = {
  login,
 
