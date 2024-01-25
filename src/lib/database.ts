import Sqlite3 from "better-sqlite3"
import getLogger from "./logger.js"
import { RedisClientType, createClient } from "redis"
import crypto from "crypto"
import colors from "colors"

import type { Database } from "better-sqlite3"

const log = getLogger("Database |", colors.yellow)

export let master: Database
export let points: Database
export let ddnet: Database

export let redis: RedisClientType = createClient()

/**
 * Tunes and configures the sqlite3 database for best performance.
 * Source: https://phiresky.github.io/blog/2020/sqlite-performance-tuning/
 * @param {string} path - The path to the sqlite3 database.
 * @returns {Sqlite3.Database}
 */
function dbOpen(path: string): Sqlite3.Database {
    try {
        /* add { verbose: log.write } to log SQL statements */
        return new Sqlite3(path, {})
    }
    catch (err) {
        log.error(`Failed to open ${path}`)
        process.exit()
    }
}

/**
 * Tunes and configures the sqlite3 database for best performance.
 * Source: https://phiresky.github.io/blog/2020/sqlite-performance-tuning/
 * @param {Sqlite3.Database} database - The sqlite3 database instance.
 */
function dbConfigure(database: Database) {
    database.pragma("journal_mode = WAL")
    database.pragma("temp_store = memory")
    database.pragma("mmap_size = 30000000000")
    database.pragma("cache_size = -500000")
    database.pragma("vacumm")
    database.pragma("optimize")
}

/**
 * Executes a given SQL query with values and returns the result.
 * @param {Sqlite3.Database} database - The sqlite3 database instance.
 * @param {string} query - The SQL query to be executed.
 * @param {Array} [values=[]] - The values to be used in the query (optional).
 * @param {boolean} returnFirst - Return only the first item in the array
 * @param {boolean} cache - Should the result be cached in Redis?
 * @returns {Promise<any>} - An array of rows as a result of the query.
 */
export async function dbQuery(database: Database, query: string, values: Array<any> = [], returnFirst: boolean = false, cache: boolean = true): Promise<any> {
    try {
        let result
        if(cache) {
            const hash = crypto
                .createHash("md5")
                .update(`${query}${JSON.stringify(values)}`)
                .digest("hex")

            const key = `cache:${hash}`

            const cache = await redis.get(key)

            if(cache === null) {
                result = database.prepare(query).all(values)
                await redis.set(key, JSON.stringify(result))
            }
            else {
                result = JSON.parse(cache)
            }
        }
        else {
            result = database.prepare(query).all(values)
        }
        if(returnFirst)
            return result[0]
        return result
    } catch (error: unknown) { 
        if(typeof error === "string")
            throw new Error(error) 
    }
}

export function dbInit() {
    /* load in db using better-sqlite3 */
    master = dbOpen(process.env.DB_MASTER ?? "../ddstats-scripts/db/master.db")
    points = dbOpen(process.env.DB_POINTS ?? "../ddstats-scripts/db/points.db")
    ddnet = dbOpen(process.env.DB_DDNET ?? "../ddstats-scripts/db/ddnet.sqlite")

    /* Configure database */
    dbConfigure(master)
    dbConfigure(points)
    dbConfigure(ddnet)
}

export async function redisInit() {
    redis.on("error", err => log.error("Redis Client Error:", err.code))
    await redis.connect()
}

export default {
    dbQuery,
    dbInit,
    redis,
    redisInit
}
