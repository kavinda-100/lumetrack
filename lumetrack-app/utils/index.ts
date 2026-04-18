import { core } from 'zod';

export const formatZodErrors = (errors: core.$ZodIssue[]): string => {
	return errors.map((error) => error.message).join(', ');
};
