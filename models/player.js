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
            SELECT date, map, player, SUM(time) FROM record_playtime 
                WHERE player = ? 
            GROUP BY date, map ORDER BY date, time DESC LIMIT ${limit};         
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