import colors, { Color } from "colors"
import { createWriteStream } from "fs"

const logWriteStream = createWriteStream(process.env.LOG_PATH ?? "./ddstats.log", { flags: "a" })
logWriteStream.write("\n\n")

export interface Logger {
    /**
     * Writes all arguments to the log...
     * @param {...any} args Writes all arguments to the log...
     */
    write: (...args: string[]) => void;

    /**
     * Logs something to the log with `ERROR!` prepended...
     * @param {...any} args Writes all arguments to the log...
     */
    error: (...args: string[]) => void;

    /**
     * Logs something to the log with `WARNING!` prepended...
     * @param {...any} args Writes all arguments to the log...
     */
    warning: (...args: string[]) => void;
}

/**
 * Creates a logger function that logs like this: "<prefix> <date + time> <message>"
 * @param {String} prefix A prefix for the logger, e.g. "MAIN", "Database", ...
 * @param {String} color  The color of the prefix, e.g. "blue", "red", ...
 * @returns {Logger} The logger function
 */

export default function getLogger(prefix: string, color: Color): Logger {
    const coloredPrefix = color(prefix)

    const write = (...args: string[]) => {
        const now = new Date(Date.now())
        const logTime = now.toLocaleString("sv-SE")
        console.log(`${coloredPrefix} ${colors.grey(logTime)} >`, ...args)
        logWriteStream.write(`${prefix.replace(/\s+\|/g, "")};${logTime};${colors.strip(args.join(" "))}\n`)
    }
    const error = (...args: string[]) => write(colors.red("ERROR!"), ...args)
    const warning = (...args: string[]) => write(colors.magenta("WARNING!"), ...args)

    return { write, error, warning }
}
