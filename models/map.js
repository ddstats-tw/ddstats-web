import { ddnet, playtime, dbQuery } from "../lib/database.js"
import { handleErrors, groupBy, splitMappers } from "../lib/misc.js"
import getLogger from "../lib/logger.js"

const log = getLogger("Database  |", "yellow")

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
        let info = dbQuery(ddnet, `
            SELECT * FROM maps WHERE map = ?;
        `, [map], true)
        info.Mapper = info.Mapper.split(/, | & /)
        return info
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
    /**
     * Search for player.
     * @param {object}  search
     * @param {string}  [search.map]
     * @param {string}  [search.mapper]
     * @returns {Array}
     */
    search: handleErrors(search => {
        console.log("GOT")
        search.mapper = search.mapper ?? "*"
        return splitMappers(dbQuery(ddnet, `
            SELECT * FROM maps WHERE map LIKE FORMAT('%%%s%', ?) AND (mapper GLOB FORMAT('%s', ?)
                OR mapper GLOB FORMAT('%s *', ?) 
                OR mapper GLOB FORMAT('* %s', ?)
                OR mapper GLOB FORMAT('%s, *', ?)
                OR mapper GLOB FORMAT('*, %s, *', ?))
        `, [search.map, search.mapper, search.mapper, search.mapper, search.mapper, search.mapper]))
    }, log),
}

export default Map