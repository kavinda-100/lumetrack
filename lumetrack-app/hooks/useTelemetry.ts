import { useEffect } from 'react';
import * as Location from 'expo-location';

export const useTelemetry = (isTracking: boolean) => {
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
						timeInterval: 3000, // 3-second heartbeats
						distanceInterval: 5,
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
