import { z } from 'zod';

const envSchema = z.object({
	NODE_ENV: z
		.enum(['development', 'production', 'test'])
		.default('development'),
	EXPO_PUBLIC_WS_URL: z.string(),
});

const env = envSchema.safeParse(process.env);

if (!env.success) {
	console.error('❌ Invalid environment variables:', env.error.message);
	throw new Error('Invalid environment variables');
}

export const ENV = env.data;
export type Env = z.infer<typeof envSchema>;

declare global {
	namespace NodeJS {
		interface ProcessEnv extends Env {}
	}
}
