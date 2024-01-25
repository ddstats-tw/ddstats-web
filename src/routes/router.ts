import { Router, Request, Response } from "express"
import leaderboardRoutes from "./leaderboards.js"
import playerRoutes from "./player.js"
import mapRoutes from "./map.js"
import miscRoutes from "./misc.js"

const routes = Router()

routes.get("/", (req: Request, res: Response) => {
    res.render("pages/home.njk",  { "search": false })
})

routes.use("/", leaderboardRoutes)
routes.use("/", playerRoutes)
routes.use("/", mapRoutes)
routes.use("/", miscRoutes)

export default routes