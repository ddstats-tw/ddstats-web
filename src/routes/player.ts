import { Router, Request, Response } from "express"
import { groupBy, Months } from "../lib/misc.js"
import Player from "../models/player.js"
import Map from "../models/map.js"

const routes = Router()

async function players_overview_route(req: Request, res: Response) {
    const player = req.params.player
    const rankings = groupBy(await Player.rankings(player), "Category")
    if(!Object.keys(rankings).length)
        return res.render("pages/player/base.njk", { "search": true })

    const lastFinishes = await Player.lastFinishes(player, 10)
    const teamPartners = await Player.teamPartners(player, 10)
    const rankedPointsGraph = await Player.rankedpointsGraph(player)
    const pointsGraph = await Player.pointsGraph(player)
    const points = await Player.points(player)
    const page = req.params.type ?? "overview"
    
    const isMapper = (await Map.search({ mapper: player })).length
    const playerinfo = await Player.playerInfo(player)
    res.render("pages/player/overview/base.njk", { page, player, playerinfo, lastFinishes, teamPartners, points, rankedPointsGraph, pointsGraph, isMapper, "search": true })
}

routes.get("/player/json", players_json)
routes.get("/player/:player", players_overview_route)
routes.get("/player/:player/:type(overview)", players_overview_route)
routes.get("/player/:player/:type(overview)/finishes", async (req: Request, res: Response) => {
    const player = req.params.player
    const lastFinishes = await Player.lastFinishes(player, 100)
    const page = req.params.type

    const isMapper = (await Map.search({ mapper: player })).length
    const playerinfo = await Player.playerInfo(player)
    res.render("pages/player/overview/finishes.njk", { player, playerinfo, page, lastFinishes, isMapper, "search": true })
})
routes.get("/player/:player/:type(overview)/partners", async (req: Request, res: Response) => {
    const player = req.params.player
    const teamPartners = await Player.teamPartners(player, 10000)
    const page = req.params.type

    const isMapper = (await Map.search({ mapper: player })).length
    const playerinfo = await Player.playerInfo(player)
    res.render("pages/player/overview/partners.njk", { player, playerinfo, page, teamPartners, isMapper, "search": true })
})

async function players_finishes_route(req: Request, res: Response) {
    const player = req.params.player
    const rankings = groupBy(await Player.rankings(player), "Category")
    if(!Object.keys(rankings).length)
        return res.render("pages/player/base.njk", { "search": true })

    const playtime = await Player.playtime(player)
    const page = req.params.type ?? "overview"

    const isMapper = (await Map.search({ mapper: player })).length
    const playerinfo = await Player.playerInfo(player)
    res.render("pages/player/finishes.njk", { page, player, playerinfo, rankings, playtime, isMapper, "search": true })
}

routes.get("/player/:player/:type(finishes)", players_finishes_route)

async function players_activity_route(req: Request, res: Response) {
    const player = req.params.player
    const rankings = groupBy(await Player.rankings(player), "Category")
    if(!Object.keys(rankings).length)
        return res.render("pages/player/base.njk", { "search": true })

    const recentPlaytime = await Player.recentPlaytime(player, 11)
    const playtimeCategories = await Player.playtimeCategories(player)
    const playtimeGametypes = await Player.playtimeGametypes(player)
    const playtimeLocation = await Player.playtimeLocation(player)
    const playtimePerMonth = await Player.playtimePerMonth(player)
    const mostPlayedMaps = await Player.mostPlayedMaps(player, 11)
    const page = req.params.type

    const isMapper = (await Map.search({ mapper: player })).length
    const recentPlayerinfo = await Player.recentPlayerinfo(player, 5)
    const playerinfo = await Player.playerInfo(player)

    res.render("pages/player/activity/base.njk", { player, playerinfo, page, recentPlayerinfo, recentPlaytime, playtimeLocation, playtimeGametypes, 
        playtimePerMonth, playtimeCategories, Months, mostPlayedMaps, isMapper, "search": true })
}

routes.get("/player/:player/:type(activity)", players_activity_route)
routes.get("/player/:player/:type(activity)/mostplayed", async (req: Request, res: Response) => {
    const player = req.params.player
    const mostPlayedMaps = await Player.mostPlayedMaps(player, 10000)
    const page = req.params.type

    const isMapper = (await Map.search({ mapper: player })).length
    const playerinfo = await Player.playerInfo(player)
    res.render("pages/player/activity/mostplayed.njk", { player, playerinfo, page, mostPlayedMaps, isMapper, "search": true })
})

routes.get("/player/:player/:type(activity)/playerinfo", async (req: Request, res: Response) => {
    const player = req.params.player
    const recentPlayerinfo = await Player.recentPlayerinfo(player, 100)
    const page = req.params.type

    const isMapper = (await Map.search({ mapper: player })).length
    const playerinfo = await Player.playerInfo(player)

    res.render("pages/player/activity/playerinfo.njk", { player, playerinfo, page, recentPlayerinfo, isMapper, "search": true })
})

routes.get("/player/:player/:type(activity)/playtime", async (req: Request, res: Response) => {
    const player = req.params.player
    const recentPlaytime = await Player.recentPlaytime(player, 1000)
    const page = req.params.type

    const isMapper = (await Map.search({ mapper: player })).length
    const playerinfo = await Player.playerInfo(player)
    res.render("pages/player/activity/playtime.njk", { player, playerinfo, page, recentPlaytime, isMapper, "search": true })
})

routes.get("/player/:player/:type(playtime)/*", async (req: Request, res: Response) => {
    res.redirect(`/player/${req.params.player}/activity`)
})

async function players_rank1s_route(req: Request, res: Response) {
    const player = req.params.player
    const rankings = groupBy(await Player.rankings(player), "Category")
    if(!Object.keys(rankings).length)
        return res.render("pages/player/base.njk", { "search": true })

    const rank1sPartners = await Player.rank1sPartners(player, 10)
    const recentTop10s = await Player.recentTop10s(player, 10)
    const AmountOfTop10Placements = await Player.AmountOfTop10Placements(player)
    const allTop10s = await Player.allTop10s(player)
    const page = req.params.type
    
    const isMapper = (await Map.search({ mapper: player })).length
    const playerinfo = await Player.playerInfo(player)
    res.render("pages/player/rank1s/base.njk", { player, playerinfo, page, allTop10s, AmountOfTop10Placements, rank1sPartners, recentTop10s, isMapper, "search": true })
}

routes.get("/player/:player/:type(rank1s)/partners", async (req: Request, res: Response) => {
    const player = req.params.player
    const page = req.params.type

    const isMapper = (await Map.search({ mapper: player })).length
    const playerinfo = await Player.playerInfo(player)
    const rank1sPartners = await Player.rank1sPartners(player, 999)
    res.render("pages/player/rank1s/partners.njk", { rank1sPartners, page, player, playerinfo, isMapper, "search": true })
})

routes.get("/player/:player/:type(rank1s)", players_rank1s_route)

async function players_json(req: Request, res: Response) {
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

    const recentPlaytime = await Player.recentPlaytime(player, 11)
    const playtimeCategories = await Player.playtimeCategories(player)
    const playtimeGametypes = await Player.playtimeGametypes(player)
    const playtimeLocation = await Player.playtimeLocation(player)
    const playtimePerMonth = await Player.playtimePerMonth(player)
    const mostPlayedMaps = await Player.mostPlayedMaps(player, 11)

    const rank1sPartners = await Player.rank1sPartners(player, 10)
    const recentTop10s = await Player.recentTop10s(player, 10)
    const AmountOfTop10Placements = await Player.AmountOfTop10Placements(player)
    const allTop10s = await Player.allTop10s(player)

    const isMapper = (await Map.search({ mapper: player })).length
    const recentPlayerinfo = await Player.recentPlayerinfo(player, 5)
    const playerinfo = await Player.playerInfo(player)
    return res.json({ player, playerinfo, points, rankings, isMapper, rankedPointsGraph, pointsGraph, playtime, recentPlayerinfo, recentPlaytime, playtimeCategories, playtimeGametypes, playtimeLocation, playtimePerMonth, mostPlayedMaps, allTop10s, AmountOfTop10Placements, rank1sPartners, recentTop10s })
}

export default routes
