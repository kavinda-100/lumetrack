import express from 'express';
import 'dotenv/config';

const app = express();
const PORT = process.env.PORT || 5004;

app.get('/api/v1/identity-service/', (req, res) => {
	res.json({ message: 'Hello from Identity Service!' });
});

app.listen(PORT, () => {
	console.log(`Server is running on port http://localhost:${PORT}`);
});
