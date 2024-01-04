import colors from "colors"
import { createWriteStream } from "fs"

const logWriteStream = createWriteStream(process.env.LOG_PATH ?? "./ddstats.log", { flags: "a" })
logWriteStream.write("\n\n")

/**
 * @typedef {Function} LoggerFunction
 * @param {*} _ Takes however many arguments and prints them to the console.
 */

/**
 * @typedef {object} Logger
 * @property {LoggerFunction} write Logs all arguments to the console.
 * @property {LoggerFunction} error Logs all arguments to the console prepended with `ERROR!`.
 */

/**
 * Creates a logger function that logs like this: "<prefix> <date + time> <message>"
 * @param {String} prefix A prefix for the logger, e.g. "MAIN", "Database", ...
 * @param {String} color  The color of the prefix, e.g. "blue", "red", ...
 * @returns {Logger} The logger function
 */
export default function getLogger(prefix, color) {
    const clr = colors[color] ? color : "blue"
    const coloredPrefix = colors[clr](prefix)

    /**
     * Writes all arguments to the log...
     * @param  {...any} args Writes all arguments to the log...
     */
    const write = (...args) => {
        const now = new Date(Date.now())
        const logTime = now.toLocaleString("sv-SE")
        console.log(`${coloredPrefix} ${colors.grey(logTime)} >`, ...args)
        logWriteStream.write(`${prefix.replace(/\s+\|/g, "")};${logTime};${colors.strip(args.join(" "))}\n`)
    }

    /**
     * Logs something to the log with `ERROR!` prepended...
     * @param  {...any} args Writes all arguments to the log...
     */
    const error = (...args) => write(colors.red("ERROR!"), ...args)

    /**
     * Logs something to the log with `WARNING!` prepended...
     * @param  {...any} args Writes all arguments to the log...
     */
    const warning = (...args) => write(colors.magenta("WARNING!"), ...args)

    return { write, error, warning }
}
