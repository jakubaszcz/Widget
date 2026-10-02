import "../App.css";
import {invoke} from "@tauri-apps/api/core";
import {getCurrentWindow} from "@tauri-apps/api/window";
import {useState} from "react";
import { Widgets } from "../widget/natives/widgets";

function App() {

    const [sizeX, setSizeX] = useState<number>(200);
    const [sizeY, setSizeY] = useState<number>(200);

    const window = getCurrentWindow();

    const [selectedWidget, setSelectedWidget] = useState<Widgets>(
        Widgets.NOTE
    );

    async function create() {
        try {
            await invoke("create_widget", {
                config: {
                    widget: selectedWidget,
                    size: {
                        x: sizeX,
                        y: sizeY
                    }
                }
            });
            window.close()
        } catch (e) {
            console.error(e)
        }
    }

    return (
        <main className="group h-screen w-full flex flex-col overflow-hidden justify-center items-center">
            <div>
                <p>size</p>
                <input
                    type="number"
                    min={1}
                    step={1}
                    required
                    value={sizeX}
                    onChange={(event) => setSizeX(event.target.valueAsNumber)}
                    placeholder="Largeur"
                    aria-label="Largeur du widget"
                    className="h-10 w-full bg-red-400 p-3 text-black"
                />
                <input
                    type="number"
                    min={1}
                    step={1}
                    required
                    value={sizeY}
                    onChange={(event) => setSizeY(event.target.valueAsNumber)}
                    placeholder="Largeur"
                    aria-label="Largeur du widget"
                    className="h-10 w-full bg-red-400 p-3 text-black"
                />
            </div>
            <div className="grid w-full grid-cols-2 gap-3 p-3">
                {Object.values(Widgets).map((type) => (
                    <button
                        key={type}
                        type="button"
                        aria-pressed={selectedWidget === type}
                        onClick={() => setSelectedWidget(type)}
                        className={`
        rounded-lg border p-4 text-left transition-colors
        focus-visible:outline-2 focus-visible:outline-violet-400
        ${
                            selectedWidget === type
                                ? "border-violet-400 bg-violet-950 text-white"
                                : "border-zinc-700 bg-zinc-900 text-zinc-300 hover:bg-zinc-800"
                        }
      `}
                    >
                        {type}
                    </button>
                ))}
            </div>
            <button onClick={() => create()}>disfidf</button>
        </main>
    );
}

export default App;
