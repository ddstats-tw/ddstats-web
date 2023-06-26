import { Router } from "express"
import leaderboardRoutes from "./leaderboards.js"
import playerRoutes from "./player.js"

const routes = Router()

const errFunc = (fn, res) => {
    return function (...args) {
        try {
            return fn(...args)
        } catch (error) {
            res.status(500).render("pages/error.njk", { error })
        }
    }
}

routes.get("/", (req, res) => {
    res.redirect("/leaderboards")
})

routes.use("/", leaderboardRoutes)
routes.use("/", playerRoutes)

export default routes