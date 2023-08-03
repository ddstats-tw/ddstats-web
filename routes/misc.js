import { Router } from "express"
import Map from "../models/map.js"
import Player from "../models/player.js"

const routes = Router()

async function search_route(req, res) {
    const query = req.query.q
    const players = await Player.search(query, 30)
    const maps = await Map.search({ map: query })
    res.render("pages/search.njk", { query, maps, players })
}

routes.get("/search", search_route)

async function search_api_route(req, res) {
    const query = req.query.q
    if(typeof query !== "string")
        return res.json({ error: "Arrays not allowed." })

    const players = await Player.search(query, 10)
    res.json(players)
}

routes.get("/search/api", search_api_route)


export default routes