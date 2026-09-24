export type DurationParts = {
    hours: number;
    minutes: number;
    seconds: number;
};

export function splitDuration(ms: number): DurationParts {
    const total = Math.max(0, Math.floor(ms / 1000));
    return {
        hours: Math.floor(total / 3600),
        minutes: Math.floor((total % 3600) / 60),
        seconds: total % 60,
    };
}

export function formatDuration(ms: number): string {
    const { hours, minutes, seconds } = splitDuration(ms);

    const mm = hours > 0 ? String(minutes).padStart(2, "0") : String(minutes);
    const ss = String(seconds).padStart(2, "0");

    return hours > 0 ? `${hours}:${mm}:${ss}` : `${mm}:${ss}`;
}

export function isoDuration(ms: number): string {
    const { hours, minutes, seconds } = splitDuration(ms);

    let out = "PT";
    if (hours > 0) out += `${hours}H`;
    if (minutes > 0) out += `${minutes}M`;
    if (seconds > 0 || out === "PT") out += `${seconds}S`;
    return out;
}
