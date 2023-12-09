import { Router } from "express"
import { groupBy, Months } from "../lib/misc.js"
import Player from "../models/player.js"

const routes = Router()

async function players_overview_route(req, res) {
    const player = req.params.player
    const rankings = groupBy(await Player.rankings(player), "Category")
    if(!Object.keys(rankings).length)
        return res.render("pages/player/base.njk")

    const rankedPointsGraph = await Player.rankedpointsGraph(player)
    const pointsGraph = await Player.pointsGraph(player)
    const playtime = await Player.playtime(player)
    const points = await Player.points(player)
    const page = req.params.type ?? "overview"

    res.render("pages/player/overview.njk", { page, player, points, rankings, rankedPointsGraph, pointsGraph, playtime })
}

routes.get("/player/json", players_json)
routes.get("/player/:player", players_overview_route)
routes.get("/player/:player/:type(overview)", players_overview_route)

async function players_playtime_route(req, res) {
    const player = req.params.player
    const rankings = groupBy(await Player.rankings(player), "Category")
    if(!Object.keys(rankings).length)
        return res.render("pages/player/base.njk")

    const recentPlaytime = await Player.recentPlaytime(player, 10)
    const playtimeCategories = await Player.playtimeCategories(player)
    const playtimeGametypes = await Player.playtimeGametypes(player)
    const playtimeLocation = await Player.playtimeLocation(player)
    const playtimePerMonth = await Player.playtimePerMonth(player)
    const mostPlayedMaps = await Player.mostPlayedMaps(player, 10)
    const page = req.params.type

    res.render("pages/player/playtime.njk", { player, page, recentPlaytime, playtimeLocation, playtimeGametypes, 
        playtimePerMonth, playtimeCategories, Months, mostPlayedMaps })
}

routes.get("/player/:player/:type(playtime)", players_playtime_route)
routes.get("/player/:player/:type(playtime)/mostplayed", async (req, res) => {
    const player = req.params.player
    const mostPlayedMaps = await Player.mostPlayedMaps(player, 10000)
    const page = req.params.type
    res.render("pages/player/mostplayed.njk", { player, page, mostPlayedMaps }) 
})
routes.get("/player/:player/:type(playtime)/activity", async (req, res) => {
    const player = req.params.player
    const recentPlaytime = await Player.recentPlaytime(player, 1000)
    const page = req.params.type
    res.render("pages/player/activity.njk", { player, page, recentPlaytime }) 
})

async function players_rank1s_route(req, res) {
    const player = req.params.player
    const rankings = groupBy(await Player.rankings(player), "Category")
    if(!Object.keys(rankings).length)
        return res.render("pages/player/base.njk")

    const rank1sPartners = await Player.rank1sPartners(player, 10)
    const recentTop10s = await Player.recentTop10s(player, 10)
    const AmountOfTop10Placements = await Player.AmountOfTop10Placements(player)
    const allTop10s = await Player.allTop10s(player)
    const page = req.params.type

    res.render("pages/player/rank1s.njk", { player, page, allTop10s, AmountOfTop10Placements, rank1sPartners, recentTop10s })
}

routes.get("/player/:player/:type(rank1s)", players_rank1s_route)

async function players_json(req, res) {
    const player = req.query.player
    const rankings = groupBy(await Player.rankings(player), "Category")
    if(!Object.keys(rankings).length)
        return res.status(404).json({
            "error": "player not found"
        })

    const rankedPointsGraph = await Player.rankedpointsGraph(player)
    const pointsGraph = await Player.pointsGraph(player)
    const playtime = await Player.playtime(player)
    const points = await Player.points(player)

    const recentPlaytime = await Player.recentPlaytime(player, 10)
    const playtimeCategories = await Player.playtimeCategories(player)
    const playtimeGametypes = await Player.playtimeGametypes(player)
    const playtimeLocation = await Player.playtimeLocation(player)
    const playtimePerMonth = await Player.playtimePerMonth(player)
    const mostPlayedMaps = await Player.mostPlayedMaps(player, 10)

    const rank1sPartners = await Player.rank1sPartners(player, 10)
    const recentTop10s = await Player.recentTop10s(player, 10)
    const AmountOfTop10Placements = await Player.AmountOfTop10Placements(player)
    const allTop10s = await Player.allTop10s(player)

    return res.json({ player, points, rankings, rankedPointsGraph, pointsGraph, playtime, recentPlaytime, playtimeCategories, playtimeGametypes, playtimeLocation, playtimePerMonth, mostPlayedMaps, allTop10s, AmountOfTop10Placements, rank1sPartners, recentTop10s })
}

export default routes