import "../App.css";
import {invoke} from "@tauri-apps/api/core";
import {getCurrentWindow} from "@tauri-apps/api/window";

function App() {

    const window = getCurrentWindow();
    async function create() {
        try {
            await invoke("create_widget");
            window.close()
        } catch (e) {
            console.error(e)
        }
    }

    return (
        <main className="group h-screen w-full flex flex-col overflow-hidden justify-center items-center">
            <button onClick={() => create()}>disfidf</button>
        </main>
    );
}

export default App;
