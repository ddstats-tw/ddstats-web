import { Router, Request, Response } from "express"
import Map from "../models/map.js"
import Player from "../models/player.js"

const routes = Router()

async function maps_overview_route(req: Request, res: Response) {
    const map = req.params.map
    const info = await Map.info(map)
    const rankings = await Map.rankings(map, 100)
    const teamrankings = await Map.teamrankings(map, 100)
    const page = req.params.type ?? "overview"

    res.render("pages/map/overview.njk", { map, page, info, rankings, teamrankings, "search": true })
    
}

routes.get("/map/:map/", maps_overview_route)
routes.get("/map/:map/:type(overview)", maps_overview_route)


async function maps_timecps_route(req: Request, res: Response) {
    const map = req.params.map
    const info = await Map.info(map)
    const timecps = await Map.timecps(map, 100)
    const page = req.params.type ?? "overview"

    res.render("pages/map/timecps.njk", { map, page, info, timecps, "search": true })
}

routes.get("/map/:map/:type(timecps)", maps_timecps_route)


async function mapper_route(req: Request, res: Response) {
    const mapper = req.params.mapper
    const maps = await Map.search({ mapper: mapper })

    const playerinfo = await Player.playerInfo(mapper)
    res.render("pages/map/mapper.njk", { maps, playerinfo, mapper, "search": true })
}

routes.get("/mapper/:mapper/", mapper_route)


export default routes
