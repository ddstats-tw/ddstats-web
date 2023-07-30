import { ddnet, playtime, dbQuery } from "../lib/database.js"
import { handleErrors, groupBy } from "../lib/misc.js"
import getLogger from "../lib/logger.js"

const log = getLogger("SQLite3 |", "yellow")

const Map = {
    categories: [
        "Any",
        "Novice",
        "Moderate",
        "Brutal",
        "Insane",
        "Dummy",
        "DDmaX.Easy",
        "DDmaX.Next",
        "DDmaX.Pro",
        "DDmaX.Nut",
        "Oldschool",
        "Solo",
        "Race",
        "Fun"
    ],
    /**
     * Get information about a map
     * @param {string} map
     * @param {integer} top - How many ranks to return?
     * @returns {Array}
     */
    info: handleErrors(map => {
        return dbQuery(ddnet, `
            SELECT * FROM maps WHERE map = ?;
        `, [map], true)
    }, log),
    /**
     * Get rankings of a map.
     * @param {string} map
     * @param {integer} top - How many ranks to return?
     * @returns {Array}
     */
    rankings: handleErrors((map, top) => {
        return dbQuery(ddnet, `
            SELECT * FROM rankings WHERE map = ? AND rank <= ?;
        `, [map, top])
    }, log),
    /**
     * Get teamrankings of a map.
     * @param {string} map
     * @param {integer} top - How many ranks to return?
     * @returns {Array}
     */
    teamrankings: handleErrors((map, top) => {
        return groupBy(dbQuery(ddnet, `
            SELECT * FROM teamrankings WHERE map = ? AND rank <= ?;
        `, [map, top]), "ID")
    }, log),
    /**
     * Get timecps of a map.
     * @param {string} map
     * @param {integer} top - How many ranks to return?
     * @returns {Array}
     */
    timecps: handleErrors((map, limit) => {
        return dbQuery(ddnet, `
            SELECT Name, cp1, cp2, cp3, cp4, cp5, cp6, cp7, cp8, time FROM race WHERE map = ? GROUP BY name ORDER BY time ASC LIMIT ${limit};
        `, [map])
    }, log),
}

export default Map