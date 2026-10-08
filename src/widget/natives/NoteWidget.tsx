import {useEffect, useRef, useState} from "react";
import {invoke} from "@tauri-apps/api/core";
import {getCurrentWindow} from "@tauri-apps/api/window";
import {openUrl} from "@tauri-apps/plugin-opener";
import Markdown from "react-markdown";
import remarkGfm from "remark-gfm";

export default function NoteWidget({inputRef, isEditing, onFinishEditing}: {
    inputRef?: React.Ref<HTMLTextAreaElement>;
    isEditing: boolean;
    onFinishEditing: () => void;
}) {
    const [text, setText] = useState("");
    const [isLoading, setIsLoading] = useState(true);
    const [loadFailed, setLoadFailed] = useState(false);
    const [isSaving, setIsSaving] = useState(false);
    const [error, setError] = useState("");
    const saving = useRef(false);
    const windowId = getCurrentWindow().label;

    useEffect(() => {
        let cancelled = false;
        async function load() {
            try {
                const response = await invoke<{content?: string} | null>("load_widget_data", {id: windowId});
                if (!cancelled) setText(response?.content ?? "");
            } catch (error) {
                console.error(error);
                if (!cancelled) {
                    setLoadFailed(true);
                    setError("Unable to load this note. Reopen the widget to try again.");
                }
            } finally {
                if (!cancelled) setIsLoading(false);
            }
        }
        void load();
        return () => { cancelled = true; };
    }, [windowId]);

    async function save() {
        if (saving.current || isLoading || loadFailed) return;
        saving.current = true;
        setIsSaving(true);
        setError("");
        try {
            // Keep the Markdown source editable; conversion is for display only.
            await invoke("save_note_widget", {id: windowId, data: {content: text}});
            onFinishEditing();
        } catch (error) {
            console.error(error);
            setError("Unable to save. Your changes are still here; please try again.");
        } finally {
            saving.current = false;
            setIsSaving(false);
        }
    }

    return (
        <div className="flex h-full min-h-0 w-full flex-col text-secondary-100">
            {isLoading ? <p className="p-3 text-sm text-secondary-400">Loading...</p> : loadFailed ? null : isEditing ? (
                <textarea
                    ref={inputRef}
                    autoFocus
                    value={text}
                    readOnly={isSaving}
                    onChange={(event) => setText(event.target.value)}
                    placeholder={"# My note\n\nWrite in **Markdown**..."}
                    aria-label="Note Markdown source"
                    className="min-h-0 w-full flex-1 resize-none bg-transparent p-3 font-mono text-sm outline-none focus-visible:ring-1 focus-visible:ring-inset focus-visible:ring-primary-500/50"
                />
            ) : (
                <article className="note-markdown min-h-0 flex-1 overflow-auto p-3" aria-label="Note preview" tabIndex={0}>
                    {text.trim() ? (
                        <Markdown remarkPlugins={[remarkGfm]} skipHtml components={{
                            a: ({href, children}) => (
                                <a href={href} onClick={(event) => {
                                    event.preventDefault();
                                    if (!href || !/^https?:\/\//i.test(href)) return;
                                    void openUrl(href).catch((error) => {
                                        console.error(error);
                                        setError("Unable to open this link.");
                                    });
                                }}>{children}</a>
                            ),
                        }}>{text}</Markdown>
                    ) : <p className="text-secondary-400">Choose Edit to write a note. Markdown is supported.</p>}
                </article>
            )}
            {error && <p role="alert" className="shrink-0 px-3 py-2 text-xs text-red-300">{error}</p>}
            {isEditing && (
                <div className="flex shrink-0 justify-end border-t border-secondary-700/50 p-2">
                    <button type="button" onClick={() => void save()} disabled={isSaving || isLoading || loadFailed}
                        className="cursor-pointer rounded-lg bg-primary px-3 py-1.5 text-xs font-semibold text-primary-950 hover:bg-primary-400 focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-primary disabled:cursor-default disabled:opacity-50">
                        {isSaving ? "Saving..." : "Save"}
                    </button>
                </div>
            )}
        </div>
    );
}
