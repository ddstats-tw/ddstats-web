import Database from "better-sqlite3"
import getLogger from "./logger.js"
import { createClient } from "redis"
import crypto from "crypto"

const log = getLogger("Database |", "yellow")

/**
 * @type {Database.Database}
 */
export let master = undefined
export let points = undefined
export let ddnet = undefined

export let redis = undefined

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
 * @param {boolean} - Return only the first item in the array
 * @returns {Promise<Array>} - An array of rows as a result of the query.
 */
export async function dbQuery(database, query, values = [], returnFirst = false, cache = true) {
    try {
        let result
        if(cache) {
            const key = crypto
                .createHash("md5")
                .update(`cache:${query}${JSON.stringify(values)}`)
                .digest("hex")

            const cache = JSON.parse(await redis.get(key))

            if(cache === null) {
                result = database.prepare(query).all(values)
                await redis.set(key, JSON.stringify(result))
            }
            else {
                result = cache
            }
        }
        else {
            result = database.prepare(query).all(values)
        }
        if(returnFirst)
            return result[0]
        return result
    } catch (error) { 
        throw new Error(error) 
    }
}

export function dbInit() {
    /* load in db using better-sqlite3 */
    master = dbOpen(process.env.DB_MASTER ?? "./db/master.db")
    points = dbOpen(process.env.DB_POINTS ?? "./db/points.db")
    ddnet = dbOpen(process.env.DB_DDNET ?? "./db/ddnet.sqlite")

    /* Configure database */
    dbConfigure(master)
    dbConfigure(points)
    dbConfigure(ddnet)
}

export async function redisInit() {
    redis = createClient()
    redis.on("error", err => log.error("Redis Client Error:", err.code))
    await redis.connect()
}

export default {
    dbQuery,
    dbInit,
    redis,
    redisInit
}
