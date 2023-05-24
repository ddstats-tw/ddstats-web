import Database from "better-sqlite3"
import getLogger from "./logger.js"

const log = getLogger("SQLite3 |", "yellow")

/**
 * @type {Database.Database}
 */
export let playtime = undefined
export let points = undefined
export let ddnet = undefined

function dbOpen(path) {
    try {
        /* add { verbose: log.write } to log SQL statements */
        return new Database(path, {})
    }
    catch (err) {
        log.error(`Failed to open ${path}`)
        process.exit()
    }
    
}

export default function dbInit() {
    /* load in db using better-sqlite3 */
    playtime = dbOpen(process.env.DB_PLAYTIME ?? "./db/playtime.db")
    points = dbOpen(process.env.DB_POINTS ?? "./db/points.db")
    ddnet = dbOpen(process.env.DB_DDNET ?? "./db/ddnet.sqlite")

    /* WAL mode */
    playtime.pragma("journal_mode = WAL")
    points.pragma("journal_mode = WAL")
    ddnet.pragma("journal_mode = WAL")
}
