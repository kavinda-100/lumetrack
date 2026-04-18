import express from 'express';
import { z } from 'zod';
import { createClient } from 'redis';

const app = express();
app.use(express.json());
const PORT = 3000;

// Initialize Redis client for later use with the Telemetry service
const redis = createClient({ url: 'redis://localhost:6379' });
redis.on('error', (err) => console.log('Redis Error', err));

const OrderSchema = z.object({
	order_id: z.string(),
	driver_id: z.string(),
	customer_id: z.string(),
});

app.post('/api/v1/orders/create', async (req, res) => {
	const order = OrderSchema.safeParse(req.body);

	if (!order.success) {
		return res.status(400).json({ error: 'Invalid order data' });
	}

	console.log(
		`✅ Order Created: ${order.data.order_id} assigned to ${order.data.driver_id}`,
	);

	res.status(201).json({
		status: 'success',
		message: 'Order initiated and tracking activated',
	});
});

app.listen(PORT, () => {
	console.log(`📦 Order Manager running on http://localhost:${PORT}`);
});
