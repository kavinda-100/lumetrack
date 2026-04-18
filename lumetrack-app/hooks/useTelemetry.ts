import { useEffect } from 'react';
import * as Location from 'expo-location';

const TIMEOUT = 3000; // 3 seconds

const useTelemetryProd = (isTracking: boolean) => {
	useEffect(() => {
		let socket: WebSocket;

		if (isTracking) {
			// Connect to our Rust Telemetry Service (via Gateway later)
			socket = new WebSocket(process.env.EXPO_PUBLIC_WS_URL);

			const startTracking = async () => {
				const { status } =
					await Location.requestForegroundPermissionsAsync();
				if (status !== 'granted') return;

				await Location.watchPositionAsync(
					{
						accuracy: Location.Accuracy.High,
						timeInterval: TIMEOUT,
						distanceInterval: 5, // Only update if the driver has moved at least 5 meters
						// Note: distanceInterval is more battery efficient than timeInterval, as it reduces unnecessary updates when the driver is stationary.
					},
					(location) => {
						const data = JSON.stringify({
							lat: location.coords.latitude,
							lng: location.coords.longitude,
							timestamp: location.timestamp,
						});

						if (socket.readyState === WebSocket.OPEN) {
							socket.send(data);
						}
					},
				);
			};

			startTracking();
		}

		return () => socket?.close();
	}, [isTracking]);
};

const useTelemetryDev = (isTracking: boolean) => {
	useEffect(() => {
		let socket: WebSocket;
		let interval: ReturnType<typeof setInterval>;

		if (isTracking) {
			socket = new WebSocket(process.env.EXPO_PUBLIC_WS_URL);

			const startHeartbeat = async () => {
				const { status } =
					await Location.requestForegroundPermissionsAsync();
				if (status !== 'granted') return;

				// Force an update every 3 seconds manually
				interval = setInterval(async () => {
					const location = await Location.getCurrentPositionAsync({
						accuracy: Location.Accuracy.High,
					});

					const data = JSON.stringify({
						lat: location.coords.latitude,
						lng: location.coords.longitude,
						timestamp: location.timestamp,
					});

					if (socket.readyState === WebSocket.OPEN) {
						console.log('📡 Sending heartbeat...');
						socket.send(data);
					}
				}, TIMEOUT);
			};

			startHeartbeat();
		}

		return () => {
			clearInterval(interval);
			socket?.close();
		};
	}, [isTracking]);
};

export const useTelemetry =
	process.env.NODE_ENV === 'production' ? useTelemetryProd : useTelemetryDev;
