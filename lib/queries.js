import { ddnet, playtime, points } from "./database.js"

function leaderboard_rank1s(type, category, sorting, page) {
    category = category == "Any" ? "%" : category
    const table = type == "rank1s" ? "rankings" : "teamrankings"
    console.log(table)

    const count = ddnet.prepare(`
        SELECT CEIL(COUNT(distinct(name))/50)+1 as total FROM (
            SELECT name FROM ${table} as rankings 
            JOIN maps ON rankings.map = maps.map 
                WHERE rank <= 5 AND maps.server LIKE ? GROUP BY name 
                    HAVING COUNT(CASE WHEN rank = ${sorting} THEN 1 END) > 0)
    `).get(category)

    const data = ddnet.prepare(`
        SELECT  RANK() OVER (ORDER BY COUNT(CASE WHEN rank = ${sorting} THEN 1 END) DESC) AS rank, Name,
                COUNT(CASE WHEN rank = 1 THEN 1 END) AS rank1s,
                COUNT(CASE WHEN rank = 2 THEN 1 END) AS rank2s,
                COUNT(CASE WHEN rank = 3 THEN 1 END) AS rank3s,
                COUNT(CASE WHEN rank = 4 THEN 1 END) AS rank4s,
                COUNT(CASE WHEN rank = 5 THEN 1 END) AS rank5s
        FROM ${table} as rankings JOIN maps ON rankings.map = maps.map 
            WHERE rank <= 5 AND maps.server LIKE ?
            GROUP BY name HAVING COUNT(CASE WHEN rank = ${sorting} THEN 1 END) > 0 
                ORDER BY ${`rank${sorting}s`} DESC LIMIT ${(page-1)*50}, 100
    `).all(category)

    return {
        page: {
            page: page,
            total: count.total
        },
        data
    }
}

function leaderboard_worsttimes(category) {
    category = category == "Any" ? "%" : category

    const data = ddnet.prepare(`
        SELECT RANK() OVER (ORDER BY worst DESC) as rank, * FROM (SELECT race.map, race.name, ROUND(race.time/60/60, 2) as worst FROM race AS race 
            JOIN mapinfo AS mapinfo ON race.map = mapinfo.map JOIN maps as maps ON maps.map = race.map
                WHERE maps.server LIKE ? AND mapinfo.BONUS = 0 ORDER BY time DESC LIMIT 100)
    `).all(category)

    return data
}

function leaderboard_mostfinishes(category) {
    category = category == "Any" ? "%" : category

    const data = ddnet.prepare(`
        SELECT race.map as map, name, count(*) AS finishes FROM race 
            JOIN maps ON race.map = maps.map 
                WHERE race.Name != 'nameless tee' AND race.Name != 'brainless tee' 
            GROUP BY race.map, name ORDER BY finishes DESC LIMIT 100;
    `).all()
    console.log(data)

    return data
}

function player_recentfinishes(player) {
    const data = ddnet.prepare(`
        SELECT race.
    `)
}


export default {
    leaderboard_rank1s,
    leaderboard_worsttimes,
    leaderboard_mostfinishes
}