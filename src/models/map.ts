import { ddnet, master, dbQuery } from "../lib/database.js"
import { handleErrors, groupBy, splitMappers } from "../lib/misc.js"
import getLogger from "../lib/logger.js"
import colors from "colors"

const log = getLogger("Database  |", colors.yellow)

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
     * @returns {Promise<Array>}
     */
    info: handleErrors(async (map: string) => {
        let info = await dbQuery(ddnet, `
            SELECT maps.*, ROUND(mediantime) as mediantime, finishes, finishesrank FROM maps JOIN mapstats ON maps.map = mapstats.map WHERE maps.map = ?;
        `, [map], true)
        if(info)
            info.Mapper = info.Mapper.split(/, | & /)
        return info
    }, log),
    /**
     * Get rankings of a map.
     * @param {string} map
     * @param {integer} top - How many ranks to return?
     * @returns {Promise<Array>}
     */
    rankings: handleErrors(async (map: string, top: number) => {
        return await dbQuery(ddnet, `
            SELECT * FROM rankings WHERE map = ? AND rank <= ?;
        `, [map, top])
    }, log),
    /**
     * Get teamrankings of a map.
     * @param {string} map
     * @param {integer} top - How many ranks to return?
     * @returns {Promise<Array>}
     */
    teamrankings: handleErrors(async (map: string, top: number) => {
        return groupBy(await dbQuery(ddnet, `
            SELECT * FROM teamrankings WHERE map = ? AND rank <= ?;
        `, [map, top]), "ID")
    }, log),
    /**
     * Get timecps of a map.
     * @param {string} map
     * @param {integer} top - How many ranks to return?
     * @returns {Promise<Array>}
     */
    timecps: handleErrors(async (map: string, limit: number) => {
        return await dbQuery(ddnet, `
            SELECT Name, cp1, cp2, cp3, cp4, cp5, cp6, cp7, cp8, time FROM race WHERE map = ? GROUP BY name ORDER BY time ASC LIMIT ${limit};
        `, [map])
    }, log),
    /**
     * Search for player.
     * @param {object}  search
     * @param {string}  [search.map]
     * @param {string}  [search.mapper]
     * @param {integer}  [search.limit]
     * @returns {Promise<Array>}
     */
    search: handleErrors(async (search: { mapper?: string; limit?: number; map: string }) => {
        search.mapper = search.mapper ?? "*"
        let maps = await dbQuery(ddnet, `
            SELECT * FROM maps WHERE map LIKE FORMAT('%%%s%', ?) AND (mapper GLOB FORMAT('%s', ?)
                OR mapper GLOB FORMAT('%s *', ?) 
                OR mapper GLOB FORMAT('* %s', ?)
                OR mapper GLOB FORMAT('%s, *', ?)
                OR mapper GLOB FORMAT('*, %s & *', ?)
                OR mapper GLOB FORMAT('*, %s, *', ?)) ${search.limit ? `LIMIT ${search.limit}` : ""}
        `, [search.map, search.mapper, search.mapper, search.mapper, search.mapper, search.mapper, search.mapper])

        for (const map in maps) {
            maps[map].Mapper = splitMappers(maps[map].Mapper)
        }
        
        return maps

    }, log),
}

export default Map
