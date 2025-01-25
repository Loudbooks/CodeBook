import type { PageServerLoad } from "./p/$types";

export const load: PageServerLoad = async () => {
	return {
		title: process.env.TITLE,
		description: process.env.DESCRIPTION,
		faviconUrl: process.env.FAVICON_URL,
		backendPort: process.env.BACKEND_PORT ? parseInt(process.env.BACKEND_PORT) : 8080,
	};
};