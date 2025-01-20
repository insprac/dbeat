/**
 * Format's a duration from seconds.
 * Example: 73 seconds => 01:13
 * Example: 3673 seconds => 01:01:13
 */
export function displayDuration(durationSeconds: number): string {
    if (durationSeconds <= 0) {
        return "00";
    }

    const hours = Math.floor(durationSeconds / 3600);
    durationSeconds -= hours * 3600;
    const minutes = Math.floor(durationSeconds / 60);
    durationSeconds -= minutes * 60;
    const seconds = Math.round(durationSeconds);

    if (hours) {
        return `${hours.toString().padStart(2, "0")}:` +
        `${minutes.toString().padStart(2, "0")}:` +
        seconds.toString().padStart(2, "0")
    } else {
        return `${minutes.toString().padStart(2, "0")}:` +
        seconds.toString().padStart(2, "0")
    }
}
