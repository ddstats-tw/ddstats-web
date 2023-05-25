import { Router } from "express"
import getLogger from "./lib/logger.js"
import queries from "./lib/queries.js"

const categories = [
    "Any",
    "Novice",
    "Moderate",
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
]

const log = getLogger("Routes       |", "blue")
const routes = Router()
const errFunc = (res, onlyLog) => err => {
    log.error(err)
    if(!onlyLog)
        res.status(500).render("pages/error.njk", { err })
}

routes.get("/", (req, res) => {
    res.redirect("/leaderboards")
})


routes.get("/leaderboards/", (req, res) => {
    res.render("pages/leaderboards.njk")
})

function rank1s_route(req, res, next) {
    const valid = [1, 2, 3, 4, 5]
    const sorting = valid.includes(Number(req.params.sort)) ? req.params.sort : 1
    const category = categories.includes(req.params.category) ? req.params.category : 'Any'
    const type = req.params.type
    const page = isNaN(req.params.page) ? 1 : Number(req.params.page)

    const leaderboard = queries.leaderboard_rank1s(type, category, sorting, page)
    res.render("pages/leaderboards/rank1s.njk", { leaderboard, sorting, type, category })
}

routes.get("/leaderboard/:type(teamrank1s|rank1s)/", rank1s_route)
routes.get("/leaderboard/:type(teamrank1s|rank1s)/category/:category/sortby/:sort", rank1s_route)
routes.get("/leaderboard/:type(teamrank1s|rank1s)/category/:category/sortby/:sort/page/:page", rank1s_route)


function worsttimes_route(req, res, next) {
    const category = categories.includes(req.params.category) ? req.params.category : 'Any'

    const leaderboard = queries.leaderboard_worsttimes(category)
    res.render("pages/leaderboards/worsttimes.njk", { leaderboard, category })
}

routes.get("/leaderboard/worsttimes/", worsttimes_route)
routes.get("/leaderboard/worsttimes/category/:category", worsttimes_route)


function mostfinishes_route(req, res, next) {
    return res.send("WIP")
    const category = categories.includes(req.params.category) ? req.params.category : 'Any'

    const leaderboard = queries.leaderboard_mostfinishes(category)
    res.render("pages/leaderboards/worsttimes.njk", { leaderboard, category })
}

routes.get("/leaderboard/finishes/", mostfinishes_route)
routes.get("/leaderboard/finishes/category/:category", mostfinishes_route)

function players_route(req, res, next) {
    const player = req.params.player
    const rankings = queries.player_rankings(player)
    res.render("pages/player/overview.njk", { rankings })
}

routes.get("/player/:player", players_route)
routes.get("/player/:player/:type(rank1s|worstranks|graphs|playtime)", players_route)




export default routes