import { Router } from "express"
import { groupBy } from "../lib/misc.js"
import Player from "../models/player.js"

const routes = Router()

function players_route(req, res) {
    console.time("database")
    const player = req.params.player
    console.time("rankings")
    const rankings = groupBy(Player.rankings(player), "Category")
    console.timeEnd("rankings")
    console.time("rankedpoints")
    const rankedPointsGraph = Player.rankedpoints(player)
    console.timeEnd("rankedpoints")
    console.time("playtime")
    const playtime = Player.playtime(player)
    console.timeEnd("playtime")
    const page = req.params.type ?? "overview"
    console.timeEnd("database")

    console.time("template")
    res.render("pages/player/overview.njk", { page, player, rankings, rankedPointsGraph, playtime })
    console.timeEnd("template")
}

routes.get("/player/:player", players_route)
routes.get("/player/:player/:type(rank1s|worstranks|graphs|playtime)", players_route)

export default routes