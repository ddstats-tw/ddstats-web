import { Router } from "express"
import Leaderboard from "../models/leaderboard.js"
import Map from "../models/map.js"

const routes = Router()

routes.get("/leaderboards/", (req, res) => {
    res.render("pages/leaderboards.njk", { "search": true })
})

async function rank1s_route(req, res) {
    const valid = [1, 2, 3, 4, 5]
    const sorting = valid.includes(Number(req.params.sort)) ? req.params.sort : 1
    const category = Map.categories.includes(req.params.category) ? req.params.category : "Any"
    const type = req.params.type
    const page = isNaN(req.params.page) ? 1 : Number(req.params.page)

    const leaderboard = await Leaderboard.rank1s(type, category, sorting, page)

    res.render("pages/leaderboards/rank1s.njk", { leaderboard, sorting, type, category, "search": true })
}

routes.get("/leaderboard/:type(teamrank1s|rank1s)/", rank1s_route)
routes.get("/leaderboard/:type(teamrank1s|rank1s)/category/:category/sortby/:sort", rank1s_route)
routes.get("/leaderboard/:type(teamrank1s|rank1s)/category/:category/sortby/:sort/page/:page", rank1s_route)


async function worsttimes_route(req, res) {
    const category = Map.categories.includes(req.params.category) ? req.params.category : "Any"

    const leaderboard = await Leaderboard.worsttimes(category)
    res.render("pages/leaderboards/worsttimes.njk", { leaderboard, category, "search": true})
}

routes.get("/leaderboard/worsttimes/", worsttimes_route)
routes.get("/leaderboard/worsttimes/category/:category", worsttimes_route)


async function mostplayed_route(req, res) {
    const category = Map.categories.includes(req.params.category) ? req.params.category : "Any"

    const leaderboard = await Leaderboard.mostplayed(category)
    res.render("pages/leaderboards/mostplayed.njk", { leaderboard, category, "search": true })
}

routes.get("/leaderboard/mostplayed/", mostplayed_route)
routes.get("/leaderboard/mostplayed/category/:category", mostplayed_route)


export default routes
