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
     * Get a players favourite partners for rank1s.
     * @param {string} player
     * @param {integer} limit - amount of entries to return.
     * @returns {Array}
     */
    rank1sPartners: handleErrors((player, limit) => {
        return dbQuery(ddnet, `
            SELECT Name, COUNT(*) as Amount FROM teamrankings WHERE id IN (
                SELECT id FROM teamrankings WHERE Name = ? AND Rank = 1 
            ) AND name != ? GROUP BY name ORDER BY COUNT(*) DESC LIMIT ${limit}
        `, [player, player])
    }, log),
    /**
     * Get recent top10s for a player.
     * @param {string} player
     * @param {integer} limit - amount of entries to return.
     * @returns {Array}
     */
    recentTop10s: handleErrors((player, limit) => {
        return dbQuery(ddnet, `
            SELECT r.timestamp, r.server, r.name, r.time, r.map, t.rank FROM rankings as r 
                LEFT JOIN teamrankings AS t 
                    ON r.name = t.name AND r.timestamp = t.timestamp 
            WHERE r.name = ? AND r.rank <= 10 ORDER BY r.timestamp DESC LIMIT ${limit}
        `, [player])
    }, log),
    /**
     * Get the amount of top10 placements
     * @param {string} player
     * @returns {Array}
     */
    AmountOfTop10Placements: handleErrors(player => {
        return dbQuery(ddnet, `
            SELECT r.Name, r.Rank, COUNT(*) AS RankAmount, TeamRankAmount FROM rankings AS r 
                LEFT JOIN (
                    SELECT Name, Rank, COUNT(*) AS TeamRankAmount FROM teamrankings 
                        WHERE Name = ? AND Rank <= 10 GROUP BY rank
                ) AS t ON r.Rank = t.Rank 
            WHERE r.Name = ? AND r.Rank <= 10 GROUP BY r.rank ORDER BY r.Rank ASC;
        `, [player, player])
    }, log),
    /**
     * Get all top10s
     * @param {string} player
     * @returns {Array}
     */
    allTop10s: handleErrors(player => {
        return dbQuery(ddnet, `
            SELECT m.Server, m.map, r.Rank as rank, rtime, t.Rank as teamrank, Ttime FROM maps AS m 
                LEFT JOIN (SELECT map, rank, time as rtime FROM rankings WHERE name = ?) as r 
                    ON r.Map = m.Map 
                LEFT JOIN (SELECT map, rank, min(time) as ttime FROM teamrankings WHERE name = ? GROUP BY map) as t 
                    ON t.Map = m.Map 
            WHERE r.Rank <= 10 OR t.Rank <= 10;
        `, [player, player])
    }, log),
}

export default Player

// SELECT strftime('%Y-%W', timestamp), SUM(points), GROUP_CONCAT(map, ', ') FROM (SELECT MIN(r.timestamp) as timestamp, r.map, m.points FROM race AS r JOIN maps AS m ON r.map = m.map WHERE name = 'deen' GROUP BY r.map) GROUP BY strftime('%Y-%W', timestamp);