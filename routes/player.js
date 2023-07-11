import { Router } from "express"
import { groupBy, Months } from "../lib/misc.js"
import Player from "../models/player.js"

const routes = Router()

function players_overview_route(req, res) {
    const player = req.params.player
    const rankings = groupBy(Player.rankings(player), "Category")
    const rankedPointsGraph = Player.rankedpoints(player)
    const playtime = Player.playtime(player)
    const page = req.params.type ?? "overview"

    res.render("pages/player/overview.njk", { page, player, rankings, rankedPointsGraph, playtime })
}

function players_playtime_route(req, res) {
    const player = req.params.player
    const recentPlaytime = Player.recentPlaytime(player, 10)
    const playtimeCategories = Player.playtimeCategories(player)
    const playtimeGametypes = Player.playtimeGametypes(player)
    const playtimeLocation = Player.playtimeLocation(player)
    const playtimePerMonth = Player.playtimePerMonth(player)
    const mostPlayedMaps = Player.mostPlayedMaps(player, 10)
    const page = req.params.type

    res.render("pages/player/playtime.njk", { player, page, recentPlaytime, playtimeLocation, playtimeGametypes, 
        playtimePerMonth, playtimeCategories, Months, mostPlayedMaps })
}

routes.get("/player/:player", players_overview_route)
routes.get("/player/:player/:type(overview)", players_overview_route)

routes.get("/player/:player/:type(playtime)", players_playtime_route)

//routes.get("/player/:player/:type(rank1s|worstranks|graphs|playtime)", players_route)

export default routes