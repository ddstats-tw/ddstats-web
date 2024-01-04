import dotenv from "dotenv"
import { dbInit, redisInit } from "./lib/database.js"
import getLogger from "./lib/logger.js"
import express from "express"
import setupExpress from "./lib/express.js"

dotenv.config()

const log = getLogger("DDStats  |", "cyan")
export const PORT = process.env.PORT ?? 12345

log.write("Loading databases...")
dbInit()
await redisInit()

log.write("Starting server...")
const server = express()
setupExpress(server)

log.write("Binding to port...")
server.listen(
    PORT,
    () => log.write(`Server is started and is listening on http://localhost:${PORT}/ !`)
)
