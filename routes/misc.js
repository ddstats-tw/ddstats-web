import { Router } from "express"
import Map from "../models/map.js"
import Player from "../models/player.js"

const routes = Router()

function search_route(req, res) {
    const query = req.query.q
    const players = Player.search(query, 20)
    const maps = Map.search({ map: query })
    res.render("pages/search.njk", { query, maps, players })
}

routes.get("/search", search_route)

function search_api_route(req, res) {
    const query = req.query.q
    const players = Player.search(query, 20)
    res.json(players)
}

routes.get("/search/api", search_api_route)


export default routes