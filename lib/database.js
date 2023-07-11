import Database from "better-sqlite3"
import getLogger from "./logger.js"

const log = getLogger("SQLite3 |", "yellow")

/**
 * @type {Database.Database}
 */
export let playtime = undefined
export let points = undefined
export let ddnet = undefined

/**
 * Tunes and configures the sqlite3 database for best performance.
 * Source: https://phiresky.github.io/blog/2020/sqlite-performance-tuning/
 * @param {string} path - The path to the sqlite3 database.
 * @returns {Database.Database}
 */
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

/**
 * Tunes and configures the sqlite3 database for best performance.
 * Source: https://phiresky.github.io/blog/2020/sqlite-performance-tuning/
 * @param {Database.Database} database - The sqlite3 database instance.
 */
function dbConfigure(database) {
    database.pragma("journal_mode = WAL")
    database.pragma("temp_store = memory")
    database.pragma("mmap_size = 30000000000")
    database.pragma("synchronous = off")
    database.pragma("cache_size = -500000")
    database.pragma("vacumm")
    database.pragma("optimize")
}

/**
 * Executes a given SQL query with values and returns the result.
 * @param {Database.Database} database - The sqlite3 database instance.
 * @param {string} query - The SQL query to be executed.
 * @param {Array} [values=[]] - The values to be used in the query (optional).
 * @returns {Array} - An array of rows as a result of the query.
 */
export function dbQuery(database, query, values = []) {
    try {
        return database.prepare(query).all(values)
    } catch (error) { 
        throw new Error(error) 
    }
}

export function dbInit() {
    /* load in db using better-sqlite3 */
    playtime = dbOpen(process.env.DB_PLAYTIME ?? "./db/playtime.db")
    points = dbOpen(process.env.DB_POINTS ?? "./db/points.db")
    ddnet = dbOpen(process.env.DB_DDNET ?? "./db/ddnet.sqlite")

    /* Configure database */
    dbConfigure(playtime)
    dbConfigure(points)
    dbConfigure(ddnet)
}

export default {
    dbQuery,
    dbInit
}
