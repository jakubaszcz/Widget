interface WidgetDefinition {
    id: string;
    title: string;
    width: number;
    height: number;
    path: string
}

export interface WidgetAction {
    type: string;
    payload?: unknown;
}

type ActionListener = (action: WidgetAction) => void | Promise<void>;

export class WistClient {
    private sockets = new Map<string, WebSocket>();
    private retries = new Map<string, ReturnType<typeof setTimeout>>();

    private listeners = new Map<string, Set<ActionListener>>();
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

    async updateWidget<T>(id: string, value: T): Promise<void> {
        const response = await fetch(`${this.endpoint.replace(/\/+$/, "")}/widgets/${encodeURIComponent(id)}/data`, {
            method: "PUT",
            headers: {
                "Content-Type": "application/json"
            },
            body: JSON.stringify(value),
            signal: AbortSignal.timeout(1000)
        });

        if (!response.ok) {
            throw new Error(`Mise à jour refusée (HTTP ${response.status}) : ${await response.text()}`);
        }
    }

    onWidgetAction(widgetId: string, callback: ActionListener): () => void {
        let callbacks = this.listeners.get(widgetId);
        if (!callbacks) {
            callbacks = new Set();
            this.listeners.set(widgetId, callbacks);
        }
        callbacks.add(callback);
        this.connectActions(widgetId);
        return () => {
            callbacks.delete(callback);
            if (callbacks.size > 0 || this.listeners.get(widgetId) !== callbacks) return;
            this.listeners.delete(widgetId);
            clearTimeout(this.retries.get(widgetId));
            this.retries.delete(widgetId);
            const socket = this.sockets.get(widgetId);
            this.sockets.delete(widgetId);
            socket?.close();
        };
    }

    private connectActions(widgetId: string): void {
        if (!this.listeners.has(widgetId) || this.sockets.has(widgetId)) return;
        clearTimeout(this.retries.get(widgetId));
        this.retries.delete(widgetId);
        const url = new URL("/events", this.endpoint);
        url.protocol = url.protocol === "https:" ? "wss:" : "ws:";
        url.searchParams.set("widgetId", widgetId);
        const socket = new WebSocket(url);
        this.sockets.set(widgetId, socket);

        socket.addEventListener("message", (event) => {
            if (this.sockets.get(widgetId) !== socket) return;
            try {
                const message = JSON.parse(event.data);
                if (message?.type !== "widget-action" || message.widgetId !== widgetId
                    || typeof message.action?.type !== "string") return;
                this.listeners.get(widgetId)?.forEach((callback) => {
                    void Promise.resolve().then(() => callback(message.action)).catch((error) => {
                        console.error("Widget action failed:", error);
                    });
                });
            } catch (error) {
                console.error("Invalid Wist action:", error);
            }
        });
        socket.addEventListener("close", () => {
            if (this.sockets.get(widgetId) !== socket) return;
            this.sockets.delete(widgetId);
            if (this.listeners.has(widgetId)) {
                this.retries.set(widgetId, setTimeout(() => this.connectActions(widgetId), 2000));
            }
        });
        socket.addEventListener("error", () => {
            console.error(`Wist action connection unavailable for ${widgetId}`);
        });
    }
}