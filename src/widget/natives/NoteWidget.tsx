import { useState } from "react";

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

    return (
        <textarea
            ref={inputRef}
            readOnly={!isEditing}
            onBlur={onFinishEditing}
            value={text}
            onChange={(event) => setText(event.target.value)}
            placeholder="Write a note."
            aria-label="Note content"
            className="h-full w-full resize-none bg-transparent p-3 text-white outline-none"
        />
    );
}
