import { Router } from "express"
import { groupBy, Months } from "../lib/misc.js"
import Player from "../models/player.js"
import Map from "../models/map.js"

const routes = Router()

async function players_overview_route(req, res) {
    const player = req.params.player
    const rankings = groupBy(await Player.rankings(player), "Category")
    if(!Object.keys(rankings).length)
        return res.render("pages/player/base.njk", { "search": true })

    const rankedPointsGraph = await Player.rankedpointsGraph(player)
    const pointsGraph = await Player.pointsGraph(player)
    const playtime = await Player.playtime(player)
    const points = await Player.points(player)
    const page = req.params.type ?? "overview"
    
    const isMapper = (await Map.search({ mapper: player })).length
    const playerinfo = await Player.playerinfo(player)
    if(playerinfo[0])
    {
        const clan = playerinfo[0].clan
        const country = playerinfo[0].country
        const skin = {
            name: playerinfo[0].skin_name,
            color_body: playerinfo[0].skin_color_body,
            color_feet: playerinfo[0].skin_color_feet,
        }

        res.render("pages/player/overview.njk", { page, player, clan, country, skin, points, rankings, rankedPointsGraph, pointsGraph, playtime, isMapper, "search": true })
    }
    else
    {
        res.render("pages/player/overview.njk", { page, player, points, rankings, rankedPointsGraph, pointsGraph, playtime, isMapper, "search": true })
    }
}

routes.get("/player/json", players_json)
routes.get("/player/:player", players_overview_route)
routes.get("/player/:player/:type(overview)", players_overview_route)

async function players_activity_route(req, res) {
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
    const playerinfo = await Player.playerinfo(player)
    if(playerinfo[0])
    {
        const clan = playerinfo[0].clan
        const country = playerinfo[0].country
        const skin = {
            name: playerinfo[0].skin_name,
            color_body: playerinfo[0].skin_color_body,
            color_feet: playerinfo[0].skin_color_feet,
        }

        res.render("pages/player/activity.njk", { player, clan, country, skin, page, recentPlayerinfo, recentPlaytime, playtimeLocation, playtimeGametypes, 
            playtimePerMonth, playtimeCategories, Months, mostPlayedMaps, isMapper, "search": true })
    } 
    else
    {
        res.render("pages/player/activity.njk", { player, page, recentPlaytime, playtimeLocation, playtimeGametypes, 
            playtimePerMonth, playtimeCategories, Months, mostPlayedMaps, isMapper, "search": true })
    }
}

routes.get("/player/:player/:type(activity)", players_activity_route)
routes.get("/player/:player/:type(activity)/mostplayed", async (req, res) => {
    const player = req.params.player
    const mostPlayedMaps = await Player.mostPlayedMaps(player, 10000)
    const page = req.params.type

    const isMapper = (await Map.search({ mapper: player })).length
    const playerinfo = await Player.playerinfo(player)
    if(playerinfo[0])
    {
        const clan = playerinfo[0].clan
        const country = playerinfo[0].country
        const skin = {
            name: playerinfo[0].skin_name,
            color_body: playerinfo[0].skin_color_body,
            color_feet: playerinfo[0].skin_color_feet,
        }

        res.render("pages/player/mostplayed.njk", { player, clan, country, skin, page, mostPlayedMaps, isMapper, "search": true })
    }
    else
    {
        res.render("pages/player/mostplayed.njk", { player, page, mostPlayedMaps, isMapper, "search": true })
    }
})
routes.get("/player/:player/:type(activity)/playerinfo", async (req, res) => {
    const player = req.params.player
    const recentPlayerinfo = await Player.recentPlayerinfo(player, 100)
    const page = req.params.type

    const isMapper = (await Map.search({ mapper: player })).length
    const playerinfo = await Player.playerinfo(player)
    if(playerinfo[0])
    {
        const clan = playerinfo[0].clan
        const country = playerinfo[0].country
        const skin = {
            name: playerinfo[0].skin_name,
            color_body: playerinfo[0].skin_color_body,
            color_feet: playerinfo[0].skin_color_feet,
        }

        res.render("pages/player/playerinfo.njk", { player, clan, country, skin, page, recentPlayerinfo, isMapper, "search": true })
    }
    else
    {
        res.render("pages/player/playerinfo.njk", { player, page, recentPlayerinfo, isMapper, "search": true })
    }
})
routes.get("/player/:player/:type(activity)/playtime", async (req, res) => {
    const player = req.params.player
    const recentPlaytime = await Player.recentPlaytime(player, 1000)
    const page = req.params.type

    const isMapper = (await Map.search({ mapper: player })).length
    const playerinfo = await Player.playerinfo(player)
    if(playerinfo[0])
    {
        const clan = playerinfo[0].clan
        const country = playerinfo[0].country
        const skin = {
            name: playerinfo[0].skin_name,
            color_body: playerinfo[0].skin_color_body,
            color_feet: playerinfo[0].skin_color_feet,
        }

        res.render("pages/player/playtime.njk", { player, clan, country, skin, page, recentPlaytime, isMapper, "search": true })
    }
    else
    {
        res.render("pages/player/playtime.njk", { player, page, recentPlaytime, isMapper, "search": true })
    }
})

async function players_rank1s_route(req, res) {
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
    const playerinfo = await Player.playerinfo(player)
    if(playerinfo[0])
    {
        const clan = playerinfo[0].clan
        const country = playerinfo[0].country
        const skin = {
            name: playerinfo[0].skin_name,
            color_body: playerinfo[0].skin_color_body,
            color_feet: playerinfo[0].skin_color_feet,
        }
    
        res.render("pages/player/rank1s.njk", { player, clan, country, skin, page, allTop10s, AmountOfTop10Placements, rank1sPartners, recentTop10s, isMapper, "search": true })
    }
    else
    {
        res.render("pages/player/rank1s.njk", { player, page, allTop10s, AmountOfTop10Placements, rank1sPartners, recentTop10s, isMapper, "search": true })
    }
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
    const playerinfo = await Player.playerinfo(player)
    if(playerinfo[0])
    {
        const clan = playerinfo[0].clan
        const country = playerinfo[0].country
        const skin = {
            name: playerinfo[0].skin_name,
            color_body: playerinfo[0].skin_color_body,
            color_feet: playerinfo[0].skin_color_feet,
        }

        return res.json({ player, clan, country, skin, points, rankings, isMapper, rankedPointsGraph, pointsGraph, playtime, recentPlayerinfo, recentPlaytime, playtimeCategories, playtimeGametypes, playtimeLocation, playtimePerMonth, mostPlayedMaps, allTop10s, AmountOfTop10Placements, rank1sPartners, recentTop10s })
    }
    else
    {
        return res.json({ player, points, rankings, isMapper, rankedPointsGraph, pointsGraph, playtime, recentPlayerinfo, recentPlaytime, playtimeCategories, playtimeGametypes, playtimeLocation, playtimePerMonth, mostPlayedMaps, allTop10s, AmountOfTop10Placements, rank1sPartners, recentTop10s })
    }
}

export default routes
