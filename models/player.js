import { ddnet, master, points, dbQuery, redis } from "../lib/database.js"
import { createIndex, handleErrors, escapeFTS } from "../lib/misc.js"
import getLogger from "../lib/logger.js"

const log = getLogger("Database  |", "yellow")

const Player = {
    /**
     * Get rankings of a player.
     * @param {string} player
     * @returns {Promise<Array>}
     */
    rankings: handleErrors(async player => {
        return await dbQuery(ddnet, `
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
     * Get recent playerinfo of a player
     * @param {string} player
     * @returns {Promise<Array>}
     */
    playerinfo: handleErrors(async player => {
        return await dbQuery(master, `
            SELECT * FROM record_snapshot
                WHERE name = ? AND skin_name != '' GROUP BY clan, country, skin_name, skin_color_body, skin_color_feet ORDER BY COUNT(*) DESC LIMIT 1
        `, [player])
    }, log),
    /**
     * Get recent playerinfo of a player
     * @param {string} player
     * @param {integer} limit - amount of entries to return.
     * @returns {Promise<Array>}
     */
    recentPlayerinfo: handleErrors(async (player, limit) => {
        return await dbQuery(master, `
            SELECT MAX(date) as lastseen, name, clan, country, skin_name, skin_color_body, skin_color_feet, SUM(time) as timeplayed FROM record_snapshot
                WHERE name = ? AND skin_name != '' GROUP BY clan, country, skin_name, skin_color_body, skin_color_feet HAVING SUM(time) > 60*60 ORDER BY lastseen DESC LIMIT ${limit}
        `, [player])
    }, log),
    /**
     * Get playtime in seconds for each map of a player.
     * @param {string} player
     * @returns {Promise<Object>}
     */
    playtime: handleErrors(async player => {
        return createIndex(await dbQuery(master, `
            SELECT map, SUM(time) as Playtime FROM record_snapshot
                WHERE name = ? GROUP BY map
        `, [player]), "map")
    }, log),
    /**
     * Get the most played maps in seconds.
     * @param {string} player
     * @param {integer} limit
     * @returns {Promise<Object>}
     */
    mostPlayedMaps: handleErrors(async (player, limit) => {
        return await dbQuery(master, `
            SELECT p.map, SUM(time) as Playtime, maps.Server FROM record_snapshot AS p
                LEFT JOIN maps ON maps.map = p.map
                WHERE name = ? GROUP BY p.map ORDER BY Playtime DESC LIMIT ${limit}           
        `, [player])
    }, log),
    /**
     * Get historical ranked points data of a player.
     * @param {string} player
     * @returns {Promise<Array>}
     */
    rankedpointsGraph: handleErrors(async player => {
        return await dbQuery(points, `
            SELECT date, rankpoints, teampoints FROM rankedpoints 
                WHERE player = ? AND strftime('%w', date) = '2'
            ORDER BY date
        `, [player])
    }, log),
    /**
     * Get historical points data of a player.
     * @param {string} player
     * @returns {Promise<Array>}
     */
    pointsGraph: handleErrors(async player => {
        let points = await dbQuery(ddnet, `
            SELECT strftime('%Y-%m-%d', timestamp) as date, SUM(points) as points, GROUP_CONCAT(map, ', ') as maps FROM (
                SELECT MIN(r.timestamp) as timestamp, r.map, m.points FROM race AS r JOIN maps AS m ON r.map = m.map WHERE name = ? GROUP BY r.map
            ) GROUP BY strftime('%Y-%W', timestamp);
        `, [player])
        points.reduce((accumulator, currentValue, currentIndex) => {
            if (currentIndex !== 0)
                currentValue.points += accumulator.points
            return currentValue
        })
        return points
    }, log),
    /**
     * Get recent playtime of a player
     * @param {string} player
     * @param {integer} limit - amount of entries to return.
     * @returns {Promise<Array>}
     */
    recentPlaytime: handleErrors(async (player, limit) => {
        return await dbQuery(master, `
            SELECT date, p.map, name, SUM(time) as Playtime, maps.Server FROM record_snapshot as p
                LEFT JOIN maps ON p.map = maps.map 
                WHERE name = ? 
            GROUP BY date, p.map ORDER BY date DESC, time DESC LIMIT ${limit};
        `, [player])
    }, log),
    /**
     * Get playtime in the different DDNet categories.
     * @param {string} player
     * @returns {Promise<Array>}
     */
    playtimeCategories: handleErrors(async (player) => {
        return await dbQuery(master, `
            SELECT Server as Category, SUM(time)/60/60 as Playtime FROM record_snapshot 
                JOIN maps ON record_snapshot.map = maps.map 
                WHERE name = ? 
            GROUP BY Category ORDER BY Category DESC
        `, [player])
    }, log),
    /**
     * Get playtime in their most played game types (CTF, DDRace, ...).
     * @param {string} player
     * @returns {Promise<Array>}
     */
    playtimeGametypes: handleErrors(async (player) => {
        return await dbQuery(master, `
            SELECT Gametype, SUM(time)/60/60 as Playtime FROM record_snapshot 
                WHERE name = ? 
            GROUP BY gametype ORDER BY Playtime DESC LIMIT 15
        `, [player])
    }, log),
    /**
     * Get playtime in their most played locations (eu, as:cn, as, na, sa, af, oc)
     * @param {string} player
     * @returns {Promise<Array>}
     */
    playtimeLocation: handleErrors(async (player) => {
        return await dbQuery(master, `
            SELECT Location, SUM(time)/60/60 as Playtime FROM record_snapshot 
                WHERE name = ? 
            GROUP BY Location ORDER BY Playtime DESC LIMIT 15
        `, [player])
    }, log),
    /**
     * Get the playtime for each month for the past 12 months.
     * @param {string} player
     * @returns {Promise<Array>}
     */
    playtimePerMonth: handleErrors(async (player) => {
        return await dbQuery(master, `
            SELECT strftime('%Y', date) AS Year, strftime('%m', date) AS Month, SUM(time)/60/60 as Playtime FROM record_snapshot 
                WHERE name = ? AND date >= date('now','-12 month') 
            GROUP BY Year, Month ORDER BY year ASC, month ASC
        `, [player])
    }, log),
    /**
     * Get a players favourite partners for rank1s.
     * @param {string} player
     * @param {integer} limit - amount of entries to return.
     * @returns {Promise<Array>}
     */
    rank1sPartners: handleErrors(async (player, limit) => {
        return await dbQuery(ddnet, `
            SELECT Name, COUNT(*) as Amount FROM teamrankings WHERE id IN (
                SELECT id FROM teamrankings WHERE Name = ? AND Rank = 1 
            ) AND name != ? GROUP BY name ORDER BY COUNT(*) DESC LIMIT ${limit}
        `, [player, player])
    }, log),
    /**
     * Get recent top10s for a player.
     * @param {string} player
     * @param {integer} limit - amount of entries to return.
     * @returns {Promise<Array>}
     */
    recentTop10s: handleErrors(async (player, limit) => {
        return await dbQuery(ddnet, `
            SELECT r.timestamp, r.server, r.name, r.time, r.map, t.rank FROM rankings as r 
                LEFT JOIN teamrankings AS t 
                    ON r.name = t.name AND r.timestamp = t.timestamp 
            WHERE r.name = ? AND r.rank <= 10 ORDER BY r.timestamp DESC LIMIT ${limit}
        `, [player])
    }, log),
    /**
     * Get the amount of top10 placements
     * @param {string} player
     * @returns {Promise<Array>}
     */
    AmountOfTop10Placements: handleErrors(async player => {
        return await dbQuery(ddnet, `
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
     * @returns {Promise<Array>}
     */
    allTop10s: handleErrors(async player => {
        return await dbQuery(ddnet, `
            SELECT m.Server, m.map, r.Rank as rank, rtime, t.Rank as teamrank, Ttime FROM maps AS m 
                LEFT JOIN (SELECT map, rank, time as rtime FROM rankings WHERE name = ?) as r 
                    ON r.Map = m.Map 
                LEFT JOIN (SELECT map, rank, min(time) as ttime FROM teamrankings WHERE name = ? GROUP BY map) as t 
                    ON t.Map = m.Map 
            WHERE r.Rank <= 10 OR t.Rank <= 10;
        `, [player, player])
    }, log),
    /**
     * Get all types of points.
     * @param {string} player
     * @returns {Promise<Array>}
     */
    points: handleErrors(async player => {
        const types = ["points", "rankpoints", "teampoints"]
        let points = {}
        for(const type of types) {
            points[type] = {}
            const rankings = await redis.hGetAll(`player:${player}:${type}`)
            const promises = []
        
            for(const category in rankings) {
                points[type][category] = {}
                points[type][category]["rank"] = rankings[category]
                const promise = redis.zScore(`leaderboard:${type}:${category}`, player)
                promises.push(promise)
            }
            
            const results = await Promise.all(promises)
            let i = 0
            points[type]["total"] = {}
            points[type]["total"]["points"] = 0
            for(const category in rankings) {
                points[type][category]["points"] = results[i]
                points[type]["total"]["points"] += results[i]
                i++
            }
        }
        return points
    }, log),
    /**
     * Search for players.
     * @param {string} query
     * @returns {Promise<Array>}
     */
    search: handleErrors(async (query, limit) => {
        return await dbQuery(ddnet, `
            SELECT Name, Points FROM players WHERE Name MATCH FORMAT('%s', ?) ORDER BY Points DESC LIMIT ${limit};
        `, [escapeFTS(query)], false, false)
    }, log),
}

export default Player
