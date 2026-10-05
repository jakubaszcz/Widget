import { useState } from "react";
import {invoke} from "@tauri-apps/api/core";
import {getCurrentWindow} from "@tauri-apps/api/window";

export default function NoteWidget({
                                       inputRef,
                                       isEditing,
                                       onFinishEditing,
                                   }: {
    inputRef?: React.Ref<HTMLTextAreaElement>;
    isEditing: boolean;
    onFinishEditing: () => void;
})  {
    const [text, setText] = useState("");
    const window = getCurrentWindow().label;

    return (
        <div>
    <textarea
        ref={inputRef}
        readOnly={!isEditing}
        value={text}
        onChange={(event) => setText(event.target.value)}
        placeholder="Write a note."
        aria-label="Note content"
        className="h-full w-full resize-none bg-transparent p-3 text-white outline-none"
    />

            {isEditing && (
                <button
                    type="button"
                    onClick={async () => {
                        try {
                            await invoke("save_note_widget", {
                                id: window,
                                data: { content: text },
                            });
                            onFinishEditing();
                        } catch (error) {
                            console.error("Erreur de sauvegarde :", error);
                        }
                    }}
                >
                    Save
                </button>
            )}
        </div>
    );
}
