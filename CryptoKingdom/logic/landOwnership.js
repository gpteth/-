const LandSchema = new mongoose.Schema({
    owner: { type: mongoose.Schema.Types.ObjectId, ref: 'User' },
    location: String,
});
const Land = mongoose.model('Land', LandSchema);

// Buy land
app.post('/buy-land', async (req, res) => {
    const { location } = req.body;
    const userId = req.user.userId; // Extracted from JWT
    const land = new Land({ owner: userId, location });
    await land.save();
    res.json({ message: 'Land purchased successfully' });
});
