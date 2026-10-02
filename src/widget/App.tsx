import "../App.css";
import {X} from "lucide-react";
import NoteWidget from "./natives/NoteWidget.tsx";
import {getCurrentWindow} from "@tauri-apps/api/window";

function App() {

    let appWindow = getCurrentWindow();

    const widgetType = new URLSearchParams(window.location.search)
        .get("widget");


    async function closeWindow() {
        try {
            await appWindow.close()
        } catch (e) {
            console.error(e)
        }
    }

  return (
      <main className="group h-screen w-full flex flex-col overflow-hidden justify-center items-center">
          <header className="flex shrink-0 items-center">
              <div className="flex h-full w-15 items-center">
                  <div
                      data-tauri-drag-region
                      className="flex h-full min-w-0 flex-1 cursor-grab items-center justify-center opacity-40 group-hover:opacity-100 duration-300 transition-all"
                  >
                      <span className="pointer-events-none h-0.5 w-full rounded-full bg-white" />
                  </div>

                  <button
                      type="button"
                      aria-label="Options du widget"
                      onClick={() => closeWindow()}
                      className="flex h-full shrink-0 cursor-pointer items-center justify-center text-white opacity-40 group-hover:opacity-100 duration-300 transition-all"
                  >
                      <X size={16} />
                  </button>
              </div>
          </header>
          <section className="min-h-0 flex-1 overflow-auto, p-3">
              <section className="min-h-0 flex-1 overflow-auto">
                  {widgetType === "note" && <NoteWidget />}
                  {widgetType === "time" && <p>Time</p>}
              </section>
          </section>
      </main>
  );
}

export default App;
