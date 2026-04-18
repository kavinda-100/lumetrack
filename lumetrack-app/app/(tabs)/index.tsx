import { useTelemetry } from '@/hooks/useTelemetry';
import React, { useState } from 'react';
import { StyleSheet, View, Text, TouchableOpacity } from 'react-native';

export default function DriverDashboard() {
	const [isOnline, setIsOnline] = useState(false);
	// When isOnline is true, the WebSocket opens and begins streaming.
	useTelemetry(isOnline);

	return (
		<View style={styles.container}>
			<Text style={styles.title}>LumeTrack Driver</Text>

			<View
				style={[
					styles.statusIndicator,
					{ backgroundColor: isOnline ? '#4ADE80' : '#F87171' },
				]}
			/>
			<Text style={styles.statusText}>
				{isOnline ? 'Streaming Live Telemetry' : 'System Offline'}
			</Text>

			<TouchableOpacity
				style={[
					styles.button,
					{ backgroundColor: isOnline ? '#EF4444' : '#3B82F6' },
				]}
				onPress={() => setIsOnline(!isOnline)}
			>
				<Text style={styles.buttonText}>
					{isOnline ? 'Stop Shift' : 'Go Online'}
				</Text>
			</TouchableOpacity>
		</View>
	);
}

const styles = StyleSheet.create({
	container: {
		flex: 1,
		alignItems: 'center',
		justifyContent: 'center',
		backgroundColor: '#0F172A',
	},
	title: {
		fontSize: 24,
		fontWeight: 'bold',
		color: 'white',
		marginBottom: 20,
	},
	statusIndicator: {
		width: 12,
		height: 12,
		borderRadius: 6,
		marginBottom: 10,
	},
	statusText: { color: '#94A3B8', marginBottom: 40 },
	button: { paddingVertical: 15, paddingHorizontal: 40, borderRadius: 12 },
	buttonText: { color: 'white', fontWeight: 'bold', fontSize: 16 },
});
