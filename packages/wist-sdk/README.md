# Wist SDK

Create a custom Windows desktop widget for your application. Send data from your app to the widget, and receive actions when users interact with it.

Your users need [Wist](https://github.com/jakubaszcz/Wist) installed and running on the same computer.

This guide builds a React widget that displays a message and sends a button click back to your app.

## 1. Install the SDK

In your application project:

```sh
npm install wist-sdk
```

This example assumes your project already uses React, React DOM, TypeScript, Vite, and `@vitejs/plugin-react`.

## 2. Create your widget

Create these files at your application project's root:

```text
widget/
    index.html
    index.tsx
    index.css
```

### `widget/index.html`

This page provides the element where React renders your widget.

```html
<!doctype html>
<html lang="en">
<head>
    <meta charset="UTF-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1" />
    <title>Test Widget</title>
</head>
<body>
    <div id="root"></div>
    <script type="module" src="./index.tsx"></script>
</body>
</html>
```

### `widget/index.tsx`

The widget receives data from Wist and sends an action when its button is clicked.

```tsx
import { useEffect, useState } from "react";
import { createRoot } from "react-dom/client";
import "./index.css";

interface WidgetData {
    test: string;
}

const initialData: WidgetData = { test: "Waiting for your app…" };

function useWistData<T>(initialValue: T): T {
    const [data, setData] = useState<T>(initialValue);

    useEffect(() => {
        function receive(event: MessageEvent) {
            if (event.source !== window.parent) return;
            if (event.data?.type !== "wist:data") return;
            if (event.data.data != null) setData(event.data.data as T);
        }

        window.addEventListener("message", receive);
        // Request the latest data after installing the listener.
        window.parent.postMessage({ type: "wist:ready" }, "*");

        return () => window.removeEventListener("message", receive);
    }, []);

    return data;
}

function Template() {
    const data = useWistData<WidgetData>(initialData);

    function notifyApp() {
        window.parent.postMessage({
            type: "wist:action",
            action: { type: "say-hello" },
        }, "*");
    }

    return (
        <main>
            <p>{data.test}</p>
            <button onClick={notifyApp}>Say hello to the app</button>
        </main>
    );
}

createRoot(document.getElementById("root")!).render(<Template />);
```

Change `WidgetData` and the JSX to display your own content. The TypeScript interface describes your data but does not validate incoming messages at runtime.

### `widget/index.css`

Add your widget's styles here. For a simple starting point:

```css
* { box-sizing: border-box; }
html, body, #root { width: 100%; height: 100%; margin: 0; overflow: hidden; }
body { font-family: system-ui, sans-serif; background: #222f40; color: #eef3f8; }
main { height: 100%; padding: 12px; display: flex; flex-direction: column;
    justify-content: center; gap: 8px; }
p { margin: 0; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }
button { align-self: flex-start; border: 0; border-radius: 6px; padding: 8px 12px;
    background: #5577a3; color: white; cursor: pointer; }
```

## 3. Build the widget

Create `vite.widget.config.ts` at your application project's root:

```ts
import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";

export default defineConfig({
    root: "widget",
    base: "./",
    plugins: [react()],
    build: {
        outDir: "../dist-widget",
    },
});
```

Run this command from that same root:

```sh
npx vite build --config vite.widget.config.ts
```

The output is `<your-project>/dist-widget`, containing `index.html` and its compiled assets. `outDir` is relative to the `widget` root. `base: "./"` keeps asset paths relative so Wist can load the copied files.

If you already use Tailwind through `@tailwindcss/vite`, add its plugin to this configuration and import Tailwind in your widget CSS. The minimal example above does not require it.

**Wist loads the build, not the original TSX.** Rebuild whenever you change your widget files.

## 4. Connect your app and display the widget

Create a shared module such as `src/api/wist.ts`. This code runs in your application, not inside the widget.

```ts
import { WistClient } from "wist-sdk";

export const WIST_WIDGET_ID = "your_app";
export const client = new WistClient();

interface WidgetData {
    test: string;
}

export async function connectWidget(): Promise<() => void> {
    if (!(await client.isAvailable())) {
        throw new Error("Start Wist before connecting your widget.");
    }

    await client.registerWidget({
        id: WIST_WIDGET_ID,
        title: "Your app",
        width: 320,
        height: 160,
        // Replace this with the absolute path to your built directory.
        path: "C:\\Users\\you\\Documents\\your-app\\dist-widget",
    });

    // App -> widget: display the initial message.
    await client.updateWidget<WidgetData>(WIST_WIDGET_ID, {
        test: "Hello from the app!",
    });

    // Widget -> app: handle the button's action.
    const unsubscribe = client.onWidgetAction(WIST_WIDGET_ID, async (action) => {
        if (action.type === "say-hello") {
            console.log("The widget button was clicked!");

            // Call your own application logic here, then update the widget.
            await client.updateWidget<WidgetData>(WIST_WIDGET_ID, {
                test: "Your app received the click!",
            });
        }
    });

    return unsubscribe;
}
```

**Call `connectWidget()` once** from your application's startup or settings flow. Defining it alone does not connect anything:

```ts
import { connectWidget } from "./api/wist"; // Adjust the import path.

let stopListening: (() => void) | undefined;

async function initializeWidget() {
    try {
        stopListening = await connectWidget();
    } catch (error) {
        console.error("Could not connect to Wist:", error);
    }
}

void initializeWidget();
// On teardown, call stopListening?.() to remove the action listener.
```

With Wist running, this registers and displays the widget. It shows **Hello from the app!**. Once the action connection opens, clicking its button logs a message in your app and changes the widget text to **Your app received the click!**.

The action connection opens asynchronously. Your app must stay running to receive actions. Avoid creating duplicate listeners: clean up the previous subscription before connecting again.

## 5. Send your own data and actions

### App -> widget

After connecting, import the shared client wherever you need it:

```ts
import { client, WIST_WIDGET_ID } from "./api/wist";

await client.updateWidget(WIST_WIDGET_ID, { test: "A new message" });
```

Send the object directly. The SDK calls `JSON.stringify` for you. Its shape should match the widget's data interface.

Each update **replaces** the previous snapshot. If you have several fields, send them together or merge partial changes in your app before sending.

### Widget -> app

The button in the example sends `say-hello`, which the app handles in `onWidgetAction`. You can choose your own action names and include data:

```ts
// Inside the widget:
window.parent.postMessage({
    type: "wist:action",
    action: { type: "set-volume", payload: { volume: 0.5 } },
}, "*");
```

In your app's callback, check `action.type` and validate `action.payload` before using it. Wist routes the action to the correct widget ID automatically. The widget does not need its own `WistClient`.

## 6. Remove the widget

When the user disables your integration, delete the widget before stopping your listener:

```ts
await client.unregisterWidget(WIST_WIDGET_ID);
stopListening?.(); // The cleanup function saved in the startup example.
```

This removes the saved registration, cached template, and runtime data, then closes the window. Deleting an already absent widget is accepted. Stopping the listener alone leaves the widget registered.

## Useful notes

- **Updating the design:** registering an existing ID does not refresh its cached files. To test a new build, unregister and register again. This resets the saved layout.
- **Widget IDs:** use 1–80 ASCII letters, digits, underscores, or hyphens. Use the same ID everywhere; do not add Wist's internal `widget_` prefix.
- **Size:** initial width must be 120–4096 and height 80–4096, both integers.
- **Paths:** pass the absolute directory containing the built `index.html`. For distribution, resolve the installed template directory instead of hardcoding a development path.
- **Tauri:** if your app has a CSP, add `http://127.0.0.1:47832` and `ws://127.0.0.1:47832` to its existing `connect-src` directive.
- **Resources:** bundle scripts, styles, images, and fonts locally. External resources and inline scripts are blocked by Wist's template policy. The Vite build uses external script files.
- **Restarting Wist:** saved widgets remain, but runtime data is lost. Resend your app's state after a restart. Action sockets reconnect automatically; data synchronization does not.
- **Limits:** JSON request bodies and serialized actions are limited to 16 KiB. Actions are not replayed after a disconnect. There is no automatic application launcher or offline-state event.
