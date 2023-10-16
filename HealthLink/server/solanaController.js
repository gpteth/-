const solanaController = async (req, res) => {
  try {
    const connection = new Connection('https://solana-api.devnet.solana.com'); // Use the appropriate network URL
    // Additional configuration options like wallets and providers can be set up here
    // ...
  } catch (error) {
    console.error(error);
    res.status(500).json({ success: false, error: 'Solana connection failed' });
  }
};
