import { Router } from "express"
import Map from "../models/map.js"

const routes = Router()

async function maps_overview_route(req, res) {
    const map = req.params.map
    const info = await Map.info(map)
    const rankings = await Map.rankings(map, 20)
    const teamrankings = await Map.teamrankings(map, 20)
    const page = req.params.type ?? "overview"

    res.render("pages/map/overview.njk", { map, page, info, rankings, teamrankings })
    
}

routes.get("/map/:map/", maps_overview_route)
routes.get("/map/:map/:type(overview)", maps_overview_route)


async function maps_timecps_route(req, res) {
    const map = req.params.map
    const info = await Map.info(map)
    const timecps = await Map.timecps(map, 20)
    const page = req.params.type ?? "overview"

    res.render("pages/map/timecps.njk", { map, page, info, timecps })
}

routes.get("/map/:map/:type(timecps)", maps_timecps_route)


async function mapper_route(req, res) {
    const mapper = req.params.mapper
    const maps = await Map.search({ mapper: mapper })
    res.render("pages/map/mapper.njk", { maps, mapper })
}

routes.get("/mapper/:mapper/", mapper_route)


export default routes