import { Router } from "express"
import { groupBy, Months } from "../lib/misc.js"
import Player from "../models/player.js"

const routes = Router()

async function players_overview_route(req, res) {
    const player = req.params.player
    const rankings = groupBy(Player.rankings(player), "Category")
    const rankedPointsGraph = Player.rankedpoints(player)
    const playtime = Player.playtime(player)
    const points = await Player.points(player)
    const page = req.params.type ?? "overview"

    res.render("pages/player/overview.njk", { page, player, points, rankings, rankedPointsGraph, playtime })
}

routes.get("/player/:player", players_overview_route)
routes.get("/player/:player/:type(overview)", players_overview_route)

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

routes.get("/player/:player/:type(playtime)", players_playtime_route)


function players_rank1s_route(req, res) {
    const player = req.params.player
    const rank1sPartners = Player.rank1sPartners(player, 10)
    const recentTop10s = Player.recentTop10s(player, 10)
    const AmountOfTop10Placements = Player.AmountOfTop10Placements(player)
    const allTop10s = Player.allTop10s(player)
    const page = req.params.type

    console.log(recentTop10s)
    res.render("pages/player/rank1s.njk", { player, page, allTop10s, AmountOfTop10Placements, rank1sPartners, recentTop10s })
}

routes.get("/player/:player/:type(rank1s)", players_rank1s_route)


export default routes