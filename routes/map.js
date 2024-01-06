import { Router } from "express"
import Map from "../models/map.js"
import Player from "../models/player.js"

const routes = Router()

async function maps_overview_route(req, res) {
    const map = req.params.map
    const info = await Map.info(map)
    const rankings = await Map.rankings(map, 20)
    const teamrankings = await Map.teamrankings(map, 20)
    const page = req.params.type ?? "overview"

    res.render("pages/map/overview.njk", { map, page, info, rankings, teamrankings, "search": true })
    
}

routes.get("/map/:map/", maps_overview_route)
routes.get("/map/:map/:type(overview)", maps_overview_route)


async function maps_timecps_route(req, res) {
    const map = req.params.map
    const info = await Map.info(map)
    const timecps = await Map.timecps(map, 20)
    const page = req.params.type ?? "overview"

    res.render("pages/map/timecps.njk", { map, page, info, timecps, "search": true })
}

routes.get("/map/:map/:type(timecps)", maps_timecps_route)


async function mapper_route(req, res) {
    const mapper = req.params.mapper
    const maps = await Map.search({ mapper: mapper })

    const playerinfo = await Player.playerinfo(mapper)
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
    
        res.render("pages/map/mapper.njk", { maps, clan, country, skin, stamp, mapper, "search": true })
    }
    else
    {
        res.render("pages/map/mapper.njk", { maps, mapper, "search": true })
    }
}

routes.get("/mapper/:mapper/", mapper_route)


export default routes
