import { useEffect, useState } from "react";

export default function TimeWidget() {
    const [time, setTime] = useState(() => new Date());

    useEffect(() => {
        const interval = setInterval(() => {
            setTime(new Date());
        }, 1000);

        return () => clearInterval(interval);
    }, []);

    return (
        <div className="flex h-full w-full items-center justify-center p-3 [container-type:inline-size]">
            <span className="text-[clamp(24px,15cqi,120px)] tabular-nums whitespace-nowrap text-white">
                {time.toLocaleTimeString("en-GB", {
                    hour: "2-digit",
                    minute: "2-digit",
                    second: "2-digit",
                    hour12: false,
                })}
            </span>
        </div>
    );
}