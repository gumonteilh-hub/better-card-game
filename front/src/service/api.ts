import { toast } from "sonner";

interface ApiFetchOptions extends RequestInit {
	showErrorToast?: boolean;
}

export async function apiFetch<T>(
	url: string,
	options: ApiFetchOptions = {},
): Promise<T> {
	const { showErrorToast = true, ...fetchOptions } = options;

	const response = await fetch(url, {
		headers: {
			"Content-Type": "application/json",
			...fetchOptions.headers,
		},
		...fetchOptions,
	});

	if (!response.ok) {
		const errorData = await response.json();
		const errorMessage = errorData.error || "Une erreur est survenue";

		console.error(`API Error (${response.status}):`, errorMessage);

		if (showErrorToast) {
			toast.error(errorMessage);
		}
	}

	// Parser et retourner le JSON
	return (await response.json()) as T;
}
