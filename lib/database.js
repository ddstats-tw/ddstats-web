import Database from "better-sqlite3"
import getLogger from "./logger.js"

const log = getLogger("SQLite3 |", "yellow")

/**
 * @type {Database.Database}
 */
export let playtime = undefined
export let points = undefined
export let ddnet = undefined

export default function dbInit() {
    /* load in db using better-sqlite3 */
    playtime = new Database('./db/playtime.db', { })
    points = new Database('./db/points.db', { })
    ddnet = new Database('./db/ddnet.sqlite', {})

    /* WAL mode */
    playtime.pragma('journal_mode = WAL')
    points.pragma('journal_mode = WAL')
    ddnet.pragma('journal_mode = WAL')

    /* Unsafe mode */
    playtime.unsafeMode()
    points.unsafeMode()
    ddnet.unsafeMode()

    log.write("Loaded in 'playtime.db'!")
    log.write("Loaded in 'points.db'!")
    log.write("Loaded in 'ddnet.sqlite'!")

    /* Repair broken indexes after inserts */
    //log.write("Running ANALYZE; ON playtime DB")
    //playtime.exec('ANALYZE')
    //log.write("ANALYZE DONE")
}
