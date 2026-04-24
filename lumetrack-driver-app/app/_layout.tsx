import { Stack } from 'expo-router';
import { StatusBar } from 'react-native';

export default function RootLayout() {
	return (
		<Stack>
			<StatusBar hidden={true} />
			<Stack.Screen name="(tabs)" options={{ headerShown: false }} />
		</Stack>
	);
}
