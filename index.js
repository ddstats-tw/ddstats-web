import dotenv from "dotenv"
import dbInit from "./lib/database.js"
import getLogger from "./lib/logger.js"
import express from "express"
import bodyParser from "body-parser"
import njk from "nunjucks"
import routes from "./routes.js"

dotenv.config()

export const PORT = process.env.PORT ?? 12345

dbInit()

const server = express()
const log = getLogger("DDStats |", "cyan")

log.write("Starting server...")
log.write("Loading middleware...")
let nunjucks = njk.configure(
    "views",
    {
        autoescape: true,
        lstripBlocks: true,
        trimBlocks: true,
        express: server,
    }
)

function ordinal(n) {
    var s = ["th", "st", "nd", "rd"]
    var v = n%100
    return (s[(v-20)%10] || s[v] || s[0])
}

nunjucks.addGlobal('ordinal', ordinal)
  
server.use("/assets", express.static("assets"))
server.use(bodyParser.urlencoded({ extended: true }))

log.write("Assembling all routes...")
server.use(routes)

log.write("Binding to port...")
server.listen(
    PORT,
    () => log.write(`Server is started and is listening on http://localhost:${PORT}/ !`)
)