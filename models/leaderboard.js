import { ddnet, master, dbQuery } from "../lib/database.js"
import { handleErrors } from "../lib/misc.js"
import getLogger from "../lib/logger.js"

const log = getLogger("Database  |", "yellow")

const Leaderboard = {
    /**
     * Get the rank1s leaderboard.
     * @param {"rank1s"|"teamrank1s"} type - The type of rank 1s. 
     * @param {string} category - Filter leaderboard by category. "Any" for no filtering.
     * @param {1|2|3|4|5} sorting - The rank to sort by.
     * @param {integer} page - The page to fetch.
     * @returns {Promise<Object>}
     */
    rank1s: handleErrors(async (type, category, sorting, page) => {
        category = category == "Any" ? "%" : category
        const table = type == "rank1s" ? "rankings" : "teamrankings"
    
        const count = await dbQuery(ddnet, `
            SELECT CEIL(COUNT(distinct(name))/50)+1 as total FROM ${table} as rankings
                JOIN maps ON rankings.map = maps.map 
                    WHERE rank = ${sorting} AND maps.server LIKE ?
        `, [category])
    
        // TODO: remove INDEXED BY
        const data = await dbQuery(ddnet, `
            SELECT  RANK() OVER (ORDER BY COUNT(CASE WHEN rank = ${sorting} THEN 1 END) DESC) AS rank, Name,
                    COUNT(CASE WHEN rank = 1 THEN 1 END) AS rank1s,
                    COUNT(CASE WHEN rank = 2 THEN 1 END) AS rank2s,
                    COUNT(CASE WHEN rank = 3 THEN 1 END) AS rank3s,
                    COUNT(CASE WHEN rank = 4 THEN 1 END) AS rank4s,
                    COUNT(CASE WHEN rank = 5 THEN 1 END) AS rank5s
            FROM ${table} as rankings INDEXED BY idx_${table}_rank_top5 JOIN maps ON rankings.map = maps.map 
                WHERE rank <= 5 AND maps.server LIKE ?
                GROUP BY name HAVING COUNT(CASE WHEN rank = ${sorting} THEN 1 END) > 0 
                    LIMIT ${(page-1)*50}, 100
        `, [category])
    
        return {
            page: {
                page: page,
                total: count.total
            },
            data
        }
    }, log),
    /**
     * Get the worst times leaderboard.
     * @param {string} category - Filter leaderboard by category. "Any" for no filtering.
     * @returns {Promise<Array>}
     */
    worsttimes: handleErrors(category => {
        category = category == "Any" ? "%" : category

        return dbQuery(ddnet, `
            SELECT RANK() OVER (ORDER BY worst DESC) as rank, * FROM (SELECT race.map, race.name, ROUND(race.time/60/60, 2) as worst FROM race AS race 
                JOIN mapinfo AS mapinfo ON race.map = mapinfo.map JOIN maps as maps ON maps.map = race.map
                    WHERE maps.server LIKE ? AND mapinfo.BONUS = 0 ORDER BY time DESC LIMIT 100)
        `, [category])
    }, log),
    /**
     * Get the most played maps leaderboard.
     * @param {string} category - Filter leaderboard by category. "Any" for no filtering.
     * @returns {Promise<Array>}
     */
    mostplayed: handleErrors(category => {
        category = category == "Any" ? "%" : category

        return dbQuery(master, `
            SELECT RANK() OVER (ORDER BY p.seconds DESC) AS rank, maps.server, p.map, p.seconds, p.mostaddicted, p.mostaddicted_seconds FROM maps_playtime AS p
                JOIN maps ON maps.map = p.map WHERE maps.server LIKE ?
            ORDER BY seconds DESC;
        `, [category])
    }, log),
}

// SELECT RANK() OVER (ORDER BY seconds DESC) as '#', map, seconds/60/60/24/365 FROM maps_playtime ORDER BY seconds DESC LIMIT 10;

export default Leaderboard
