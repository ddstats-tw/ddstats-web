import { ddnet, playtime, points, dbQuery } from "../lib/database.js"
import { createIndex, handleErrors } from "../lib/misc.js"
import getLogger from "../lib/logger.js"

const log = getLogger("SQLite3 |", "yellow")

const Player = {
    /**
     * Get rankings of a player.
     * @param {string} player
     * @returns {Array}
     */
    rankings: handleErrors(player => {
        return dbQuery(ddnet, `
            SELECT rankings.map, rankings.name, rankings.time, rankings.map, rankings.server, 
                rankings.rank, min(teamrankings.rank) as Teamrank, maps.Server as Category, maps.Points FROM rankings as rankings
                JOIN maps AS maps 
                    ON maps.map = rankings.map
                LEFT JOIN teamrankings AS teamrankings 
                    ON rankings.map = teamrankings.map AND rankings.name = teamrankings.name 
            WHERE rankings.name = ? GROUP BY rankings.map
        `, [player])
    }, log),
    /**
     * Get playtime in seconds for each map of a player.
     * @param {string} player
     * @returns {Object}
     */
    playtime: handleErrors(player => {
        return createIndex(dbQuery(playtime, `
            SELECT map, SUM(time) as Playtime FROM record_playtime
                WHERE player = ? GROUP BY map            
        `, [player]), "map")
    }, log),
    /**
     * Get the most played maps in seconds.
     * @param {string} player
     * @param {integer} limit
     * @returns {Object}
     */
    mostPlayedMaps: handleErrors((player, limit) => {
        return dbQuery(playtime, `
            SELECT map, SUM(time) as Playtime FROM record_playtime
                WHERE player = ? GROUP BY map ORDER BY Playtime DESC LIMIT ${limit}           
        `, [player])
    }, log),
    /**
     * Get historical ranked points data of a player.
     * @param {string} player
     * @returns {Array}
     */
    rankedpoints: handleErrors(player => {
        return dbQuery(points, `
            SELECT date, rankpoints, teampoints FROM rankedpoints 
                WHERE player = ? AND strftime('%w', date) = '2'
            ORDER BY date
        `, [player])
    }, log),
    /**
     * Get recent playtime of a player
     * @param {string} player
     * @param {integer} limit - amount of entries to return.
     * @returns {Array}
     */
    recentPlaytime: handleErrors((player, limit) => {
        return dbQuery(playtime, `
            SELECT date, map, player, SUM(time) as Playtime FROM record_playtime 
                WHERE player = ?
            GROUP BY date, map ORDER BY date DESC, time DESC LIMIT ${limit};
        `, [player])
    }, log),
    /**
     * Get playtime in the different DDNet categories.
     * @param {string} player
     * @returns {Array}
     */
    playtimeCategories: handleErrors((player) => {
        return dbQuery(playtime, `
            SELECT Server as Category, SUM(time)/60/60 as Playtime FROM record_playtime 
                JOIN maps ON record_playtime.map = maps.map 
                WHERE player = ? 
            GROUP BY Category ORDER BY Category DESC
        `, [player])
    }, log),
    /**
     * Get playtime in their most played game types (CTF, DDRace, ...).
     * @param {string} player
     * @returns {Array}
     */
    playtimeGametypes: handleErrors((player) => {
        return dbQuery(playtime, `
            SELECT Gametype, SUM(time)/60/60 as Playtime FROM record_playtime 
                WHERE player = ? 
            GROUP BY gametype ORDER BY Playtime DESC LIMIT 15
        `, [player])
    }, log),
    /**
     * Get playtime in their most played locations (eu, as:cn, as, na, sa, af, oc)
     * @param {string} player
     * @returns {Array}
     */
    playtimeLocation: handleErrors((player) => {
        return dbQuery(playtime, `
            SELECT Location, SUM(time)/60/60 as Playtime FROM record_playtime 
                WHERE player = ? 
            GROUP BY Location ORDER BY Playtime DESC LIMIT 15
        `, [player])
    }, log),
    /**
     * Get the playtime for each month for the past 12 months.
     * @param {string} player
     * @returns {Array}
     */
    playtimePerMonth: handleErrors((player) => {
        return dbQuery(playtime, `
            SELECT strftime('%Y', date) AS Year, strftime('%m', date) AS Month, SUM(time)/60/60 as Playtime FROM record_playtime 
                WHERE player = ? AND date >= date('now','-12 month') 
            GROUP BY Year, Month ORDER BY year ASC, month ASC
        `, [player])
    }, log),
    /**
     * Get recent finishes of a player
     * @param {string} player
     * @param {integer} limit - amount of entries to return.
     * @returns {Array}
     */
    recentFinishes: handleErrors((player, limit) => {
        return dbQuery(ddnet, `
            SELECT race.timestamp, race.map, race.time, race.server, rankings.rank FROM race 
                LEFT JOIN rankings ON race.name = rankings.name 
                    AND race.map = rankings.map 
                    AND race.timestamp = rankings.timestamp 
                    AND race.time = rankings.time 
                WHERE race.name = ? 
            ORDER BY race.timestamp DESC LIMIT ${limit}
        `, [player])
    }, log),
}

export default Player