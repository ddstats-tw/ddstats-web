import { Router } from "express"
import { groupBy, Months } from "../lib/misc.js"
import Player from "../models/player.js"

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
        const stamp = playerinfo[0].date

        res.render("pages/player/overview.njk", { page, player, clan, country, skin, stamp, points, rankings, rankedPointsGraph, pointsGraph, playtime, "search": true })
    }
    else
    {
        res.render("pages/player/overview.njk", { page, player, points, rankings, rankedPointsGraph, pointsGraph, playtime, "search": true })
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

    const recentPlaytime = await Player.recentPlaytime(player, 10)
    const playtimeCategories = await Player.playtimeCategories(player)
    const playtimeGametypes = await Player.playtimeGametypes(player)
    const playtimeLocation = await Player.playtimeLocation(player)
    const playtimePerMonth = await Player.playtimePerMonth(player)
    const mostPlayedMaps = await Player.mostPlayedMaps(player, 10)
    const page = req.params.type

    const recentPlayerinfo = await Player.recentPlayerinfo(player, 3)
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
        const stamp = playerinfo[0].date

        res.render("pages/player/activity.njk", { player, clan, country, skin, stamp, page, recentPlayerinfo, recentPlaytime, playtimeLocation, playtimeGametypes, 
            playtimePerMonth, playtimeCategories, Months, mostPlayedMaps, "search": true })
    } 
    else
    {
        res.render("pages/player/activity.njk", { player, page, recentPlaytime, playtimeLocation, playtimeGametypes, 
            playtimePerMonth, playtimeCategories, Months, mostPlayedMaps, "search": true })
    }
}

routes.get("/player/:player/:type(activity)", players_activity_route)
routes.get("/player/:player/:type(activity)/mostplayed", async (req, res) => {
    const player = req.params.player
    const mostPlayedMaps = await Player.mostPlayedMaps(player, 10000)
    const page = req.params.type

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
        const stamp = playerinfo[0].date
        res.render("pages/player/mostplayed.njk", { player, clan, country, skin, stamp, page, mostPlayedMaps, "search": true })
    }
    else
    {
        res.render("pages/player/mostplayed.njk", { player, page, mostPlayedMaps, "search": true })
    }
})
routes.get("/player/:player/:type(activity)/playerinfo", async (req, res) => {
    const player = req.params.player
    const recentPlayerinfo = await Player.recentPlayerinfo(player, 1000)
    const page = req.params.type

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
        const stamp = playerinfo[0].date
        res.render("pages/player/playerinfo.njk", { player, clan, country, skin, stamp, page, recentPlayerinfo, "search": true })
    }
    else
    {
        res.render("pages/player/playerinfo.njk", { player, page, recentPlayerinfo, "search": true })
    }
})
routes.get("/player/:player/:type(activity)/playtime", async (req, res) => {
    const player = req.params.player
    const recentPlaytime = await Player.recentPlaytime(player, 1000)
    const page = req.params.type

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
        const stamp = playerinfo[0].date
        res.render("pages/player/playtime.njk", { player, clan, country, skin, stamp, page, recentPlaytime, "search": true })
    }
    else
    {
        res.render("pages/player/playtime.njk", { player, page, recentPlaytime, "search": true })
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
        const stamp = playerinfo[0].date
    
        res.render("pages/player/rank1s.njk", { player, clan, country, skin, stamp, page, allTop10s, AmountOfTop10Placements, rank1sPartners, recentTop10s, "search": true })
    }
    else
    {
        res.render("pages/player/rank1s.njk", { player, page, allTop10s, AmountOfTop10Placements, rank1sPartners, recentTop10s, "search": true })
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

    const recentPlayerinfo = await Player.recentPlayerinfo(player, 10)
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
        const stamp = playerinfo[0].date
    
        return res.json({ player, clan, country, skin, stamp, points, rankings, rankedPointsGraph, pointsGraph, playtime, recentPlayerinfo, recentPlaytime, playtimeCategories, playtimeGametypes, playtimeLocation, playtimePerMonth, mostPlayedMaps, allTop10s, AmountOfTop10Placements, rank1sPartners, recentTop10s })
    }
    else
    {
        return res.json({ player, points, rankings, rankedPointsGraph, pointsGraph, playtime, recentPlayerinfo, recentPlaytime, playtimeCategories, playtimeGametypes, playtimeLocation, playtimePerMonth, mostPlayedMaps, allTop10s, AmountOfTop10Placements, rank1sPartners, recentTop10s })

    }
}

export default routes
