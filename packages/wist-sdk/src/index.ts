interface WidgetDefinition {
    id: string;
    title: string;
    width: number;
    height: number;
    path: string
}

export class WistClient {
    constructor(private readonly endpoint: string = "http://127.0.0.1:47832") {}

    async isAvailable(): Promise<boolean> {
        try {
            const response = await fetch(`${this.endpoint.replace(/\/+$/, "")}/health`, {
                cache: "no-store",
                signal: AbortSignal.timeout(1500),
            });

            if (!response.ok) return false;

            const body: unknown = await response.json();

            return (
                typeof body === "object" &&
                body !== null &&
                "service" in body &&
                body.service === "wist"
            );
        } catch {
            return false;
        }
    }

    async registerWidget(widget: WidgetDefinition): Promise<void> {
        const response = await fetch(`${this.endpoint.replace(/\/+$/, "")}/widgets`, {
            method: "POST",
            headers: {
                "Content-Type": "application/json"
            },
            body: JSON.stringify(widget),
            signal: AbortSignal.timeout(5000)
        });

        if (!response.ok) {
            const details: unknown = await response.json().catch(() => null);
            const message = typeof details === "object" && details !== null
                && "error" in details && typeof details.error === "string"
                ? ` : ${details.error}`
                : "";
            throw new Error(
                `Wist a refusé le widget : HTTP ${response.status}${message}`,
            );
        }
    }
}
