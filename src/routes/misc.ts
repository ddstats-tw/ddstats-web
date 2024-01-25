import { Router, Request, Response } from "express"
import Map from "../models/map.js"
import Player from "../models/player.js"

const routes = Router()

async function search_route(req: Request, res: Response) {
    const query = req.query.q
    if(typeof query !== "string")
        return res.json({ error: "Arrays not allowed." })

    const players = await Player.search(query, 30)
    const maps = await Map.search({ map: query })
    res.render("pages/search.njk", { query, maps, players })
}

routes.get("/search", search_route)

async function search_api_route(req: Request, res: Response) {
    const query = req.query.q
    if(typeof query !== "string")
        return res.json({ error: "Arrays not allowed." })

    const maps = await Map.search({ map: query, limit: 5 })
    const players = await Player.search(query, 5)
    res.json({ players, maps })
}

routes.get("/search/api", search_api_route)

routes.get("/faq", (req: Request, res: Response) => { res.render("pages/faq.njk"), {"search": true }})
routes.get("/status-WIP", (req: Request, res: Response) => { res.render("pages/status.njk"), { "search": true }})

export default routes
