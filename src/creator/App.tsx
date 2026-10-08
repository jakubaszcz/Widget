import "../App.css";
import logo from "../../logo.png";
import {invoke} from "@tauri-apps/api/core";
import {getCurrentWindow} from "@tauri-apps/api/window";
import {useRef, useState, type FormEvent} from "react";
import {Check, Clock3, LoaderCircle, Plus, StickyNote, X} from "lucide-react";
import {Widgets} from "../widget/natives/widgets";

const DEFAULT_SIZE = {x: 200, y: 200};
const choices = [
    {type: Widgets.NOTE, name: "Note", description: "Keep your ideas close.", icon: StickyNote},
    {type: Widgets.TIME, name: "Clock", description: "Time at a glance.", icon: Clock3},
];

function App() {
    const [selectedWidget, setSelectedWidget] = useState<Widgets>(Widgets.NOTE);
    const [isCreating, setIsCreating] = useState(false);
    const [isCreated, setIsCreated] = useState(false);
    const [error, setError] = useState("");
    const creating = useRef(false);
    const busy = isCreating || isCreated;

    async function close() {
        try {
            await getCurrentWindow().close();
        } catch (error) {
            console.error(error);
            setError("Unable to close this window. Please try again.");
        }
    }

    async function create(event: FormEvent<HTMLFormElement>) {
        event.preventDefault();
        if (creating.current) return;
        creating.current = true;
        setIsCreating(true);
        setError("");
        try {
            await invoke("create_widget", {
                config: {widget: selectedWidget, size: DEFAULT_SIZE},
            });
        } catch (error) {
            console.error(error);
            setError("Unable to create your widget. Please try again.");
            creating.current = false;
            setIsCreating(false);
            return;
        }
        setIsCreated(true);
        setIsCreating(false);
        await close();
    }

    return (
        <main className="flex h-dvh w-full flex-col overflow-y-auto border border-secondary-800 bg-secondary-900 font-sans text-secondary-100">
            <header data-tauri-drag-region className="flex h-11 shrink-0 items-center justify-between border-b border-secondary-800 px-5">
                <span className="pointer-events-none flex items-center gap-2 text-xs font-medium text-secondary-400">
                    <img src={logo} alt="Logo" className="size-[14px]" /> Widget Creator
                </span>
                <button type="button" aria-label="Close" onClick={() => void close()} className="flex size-7 cursor-pointer items-center justify-center rounded-lg text-secondary-400 transition-colors hover:bg-secondary-800 hover:text-secondary-100 focus-visible:outline-2 focus-visible:outline-primary motion-reduce:transition-none">
                    <X size={16} />
                </button>
            </header>

            <form onSubmit={create} className="flex flex-1 flex-col gap-5 p-5">
                <div>
                    <h1 className="text-xl font-semibold tracking-tight">Make room for the essentials.</h1>
                    <p className="mt-1 text-xs text-secondary-400">Choose a widget to add to your desktop.</p>
                </div>

                <fieldset disabled={busy} className="min-w-0">
                    <legend className="mb-2.5 text-xs font-medium text-secondary-300">Your widgets</legend>
                    <div className="grid grid-cols-2 gap-3">
                        {choices.map(({type, name, description, icon: Icon}) => {
                            const selected = selectedWidget === type;
                            return (
                                <button key={type} type="button" aria-pressed={selected} onClick={() => setSelectedWidget(type)}
                                    className={`relative flex min-w-0 cursor-pointer flex-col items-start rounded-xl border p-4 text-left transition-colors focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-primary disabled:cursor-default disabled:opacity-60 motion-reduce:transition-none ${selected ? "border-primary-500/70 bg-primary-950 text-primary-100" : "border-secondary-700 bg-secondary-800/40 text-secondary-200 hover:border-secondary-500 hover:bg-secondary-800"}`}>
                                    <span className={`mb-3 flex size-9 items-center justify-center rounded-lg ${selected ? "bg-primary-500/15 text-primary-300" : "bg-secondary-700/50 text-secondary-300"}`}><Icon size={19} /></span>
                                    {selected && <Check size={15} className="absolute top-4 right-4 text-primary-400" />}
                                    <span className="text-sm font-semibold">{name}</span>
                                    <span className="mt-1 text-xs leading-relaxed text-secondary-400">{description}</span>
                                </button>
                            );
                        })}
                    </div>
                </fieldset>

                {error && <p role="alert" className="text-xs text-red-300">{error}</p>}
                <footer className="mt-auto flex items-center justify-between gap-3 border-t border-secondary-800 pt-4">
                    <button type="button" onClick={() => void close()} className="cursor-pointer rounded-lg px-3 py-2 text-xs font-medium text-secondary-400 transition-colors hover:bg-secondary-800 hover:text-secondary-100 focus-visible:outline-2 focus-visible:outline-primary motion-reduce:transition-none">Cancel</button>
                    <button type="submit" disabled={busy} className="inline-flex cursor-pointer items-center justify-center gap-2 rounded-lg bg-primary px-4 py-2.5 text-xs font-semibold text-primary-950 transition-colors hover:bg-primary-400 active:bg-primary-600 focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-primary-300 disabled:cursor-default disabled:opacity-60 motion-reduce:transition-none">
                        {isCreating ? <LoaderCircle size={15} className="animate-spin motion-reduce:animate-none" /> : isCreated ? <Check size={15} /> : <Plus size={15} />}
                        {isCreating ? "Creating..." : isCreated ? "Widget created" : "Create widget"}
                    </button>
                </footer>
            </form>
        </main>
    );
}

export default App;
