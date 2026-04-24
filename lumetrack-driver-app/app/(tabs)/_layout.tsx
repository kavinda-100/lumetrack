import { Ionicons } from '@expo/vector-icons';
import { Tabs } from 'expo-router';

export default function TabLayout() {
	return (
		<Tabs
			screenOptions={{
				tabBarShowLabel: false,
				tabBarActiveTintColor: '#AB8BFF',
				tabBarInactiveTintColor: '#888',
				tabBarItemStyle: {
					width: '100%',
					height: '100%',
					justifyContent: 'center',
					alignItems: 'center',
					paddingVertical: 10,
				},
				tabBarStyle: {
					backgroundColor: '#0f0D23',
					borderRadius: 20,
					marginHorizontal: 10,
					marginBottom: 40,
					height: 30,
					position: 'absolute',
					overflow: 'hidden',
					borderWidth: 1,
					borderColor: '#0f0D23',
				},
			}}
		>
			<Tabs.Screen
				name="index"
				options={{
					headerShown: false,
					title: 'Home',
					tabBarIcon(props) {
						return (
							<Ionicons
								name="home"
								size={props.size}
								color={props.color}
							/>
						);
					},
				}}
			/>
		</Tabs>
	);
}
