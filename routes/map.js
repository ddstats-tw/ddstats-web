import { Router } from "express"
import { groupBy, Months } from "../lib/misc.js"
import Map from "../models/map.js"

const routes = Router()

function maps_overview_route(req, res) {
    const map = req.params.map
    const info = Map.info(map)
    const rankings = Map.rankings(map, 20)
    const teamrankings = Map.teamrankings(map, 20)
    const page = req.params.type ?? "overview"

    res.render("pages/map/overview.njk", { map, page, info, rankings, teamrankings })
}

routes.get("/map/:map/", maps_overview_route)
routes.get("/map/:map/:type(overview)", maps_overview_route)


function maps_timecps_route(req, res) {
    const map = req.params.map
    const info = Map.info(map)
    const timecps = Map.timecps(map, 20)
    const page = req.params.type ?? "overview"

    res.render("pages/map/timecps.njk", { map, page, info, timecps })
}

routes.get("/map/:map/:type(timecps)", maps_timecps_route)


function mapper_route(req, res) {
    const mapper = req.params.mapper
    const maps = Map.search({ mapper: mapper })
    res.render("pages/map/mapper.njk", { maps, mapper })
}

routes.get("/mapper/:mapper/", mapper_route)


export default routes