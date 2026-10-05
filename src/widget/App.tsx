import "../App.css";
import {Ellipsis, Pencil, X} from "lucide-react";
import NoteWidget from "./natives/NoteWidget.tsx";
import {getCurrentWindow} from "@tauri-apps/api/window";
import TimeWidget from "./natives/TimeWidget.tsx";
import {useEffect, useRef, useState} from "react";

function App() {
    const [optionsOpen, setOptionsOpen] = useState(false);
    const optionsRef = useRef<HTMLDivElement>(null);
    const optionsButtonRef = useRef<HTMLButtonElement>(null);
    const noteRef = useRef<HTMLTextAreaElement>(null);
    const restoreOptionsFocus = useRef(false);
    const [isEditing, setIsEditing] = useState(false);
    const iconButtonClass = "flex h-5 w-5 shrink-0 cursor-pointer items-center justify-center rounded text-white opacity-40 transition-opacity duration-300 group-hover:opacity-100 focus-visible:opacity-100 focus-visible:outline focus-visible:outline-1 focus-visible:outline-white/60";

    useEffect(() => {
        if (!optionsOpen) {
            if (restoreOptionsFocus.current) {
                optionsButtonRef.current?.focus();
                restoreOptionsFocus.current = false;
            }
            return;
        }
        optionsRef.current?.querySelector<HTMLButtonElement>("#widget-options button")?.focus();
        function handlePointerDown(event: PointerEvent) {
            if (event.target instanceof Node && !optionsRef.current?.contains(event.target)) {
                setOptionsOpen(false);
            }
        }
        function handleKeyDown(event: KeyboardEvent) {
            if (event.key === "Escape") {
                restoreOptionsFocus.current = true;
                setOptionsOpen(false);
            }
        }
        document.addEventListener("pointerdown", handlePointerDown);
        document.addEventListener("keydown", handleKeyDown);
        return () => {
            document.removeEventListener("pointerdown", handlePointerDown);
            document.removeEventListener("keydown", handleKeyDown);
        };
    }, [optionsOpen]);

    let appWindow = getCurrentWindow();

    const widgetType = new URLSearchParams(window.location.search)
        .get("widget");


    async function closeWindow() {
        setOptionsOpen(false);
        try {
            await appWindow.close()
        } catch (e) {
            console.error(e)
        }
    }

  return (
      <main className="group h-screen w-full flex flex-col overflow-hidden justify-center items-center">
          <header className="flex shrink-0 items-center">
              <div
                  ref={optionsRef}
                  onBlur={(event) => {
                      if (!event.currentTarget.contains(event.relatedTarget)) setOptionsOpen(false);
                  }}
                  className="relative flex h-6 w-fit shrink-0 items-center"
              >
                  <div
                      data-tauri-drag-region
                      className="flex h-full w-10 shrink-0 cursor-grab items-center justify-center opacity-40 group-hover:opacity-100 duration-300 transition-opacity"
                  >
                      <span className="pointer-events-none h-0.5 w-full rounded-full bg-white" />
                  </div>

                  <button
                      ref={optionsButtonRef}
                      type="button"
                      aria-label="Options du widget"
                      aria-expanded={optionsOpen}
                      aria-controls="widget-options"
                      onClick={() => setOptionsOpen((open) => !open)}
                      className={iconButtonClass}
                  >
                      <Ellipsis size={16} />
                  </button>

                  {optionsOpen && (
                      <div
                          id="widget-options"
                          role="group"
                          aria-label="Options du widget"
                          className="absolute left-1/2 top-full z-50 mt-1 flex w-32 -translate-x-1/2 flex-col gap-1 rounded-xl border border-white/10 bg-neutral-900/95 p-1 text-white shadow-lg"
                      >
                          {widgetType === "note" && (
                              <button
                                  type="button"
                                  onClick={() => {
                                      setOptionsOpen(false);
                                      setIsEditing(true)
                                      noteRef.current?.focus();
                                  }}
                                  className="flex cursor-pointer items-center gap-2 rounded-lg px-3 py-2 text-xs transition-colors hover:bg-white/10 focus-visible:bg-white/10 focus-visible:outline-none"
                              >
                                  <Pencil size={14} />
                                  Edit
                              </button>
                          )}

                          <button
                              type="button"
                              onClick={closeWindow}
                              className="flex cursor-pointer items-center gap-2 rounded-lg px-3 py-2 text-xs transition-colors hover:bg-white/10 focus-visible:bg-white/10 focus-visible:outline-none"
                          >
                              <X size={14} />
                              Close
                          </button>
                      </div>
                  )}
              </div>
          </header>
          <section className="min-h-0 flex-1 overflow-auto, p-3d">
              <section className="min-h-0 flex-1 overflow-auto">
                  {widgetType === "note" && <NoteWidget
                      inputRef={noteRef}
                      isEditing={isEditing}
                      onFinishEditing={() => setIsEditing(false)}
                  />}
                  {widgetType === "time" && <TimeWidget />}
              </section>
          </section>
      </main>
  );
}

export default App;
