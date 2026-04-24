import { z } from 'zod';
import { formatZodErrors } from './utils';

export const envSchema = z.object({
	EXPO_PUBLIC_GATEWAY_URL: z.url({
		error: 'EXPO_PUBLIC_GATEWAY_URL must be a valid URL',
	}),
	EXPO_PUBLIC_WS_URL: z.url({
		error: 'EXPO_PUBLIC_WS_URL must be a valid URL',
	}),
});

const validatedEnv = envSchema.safeParse(process.env);

if (!validatedEnv.success) {
	console.error(
		'Invalid environment variables:',
		formatZodErrors(validatedEnv.error.issues),
	);
	throw new Error('Environment variable validation failed');
}

export const env = validatedEnv.data;

export type Env = z.infer<typeof envSchema>;

// update global type definitions to include our validated env
declare global {
	namespace NodeJS {
		// eslint-disable-next-line @typescript-eslint/no-empty-object-type
		interface ProcessEnv extends Env {}
	}
}
