import bodyParser from "body-parser"
import njk from "nunjucks"
import express from "express"
import macros from "./macros.js"
import router from "../routes/router.js"
import { Express } from "express-serve-static-core"
import fs from 'fs'
import crypto from 'crypto'

/**
 * This function sets up express and nunjucks.
 * @param {Express} server
 */
export default function setupExpress(server: Express) {
    let nunjucks = njk.configure(
        "views",
        {
            autoescape: true,
            lstripBlocks: true,
            trimBlocks: true,
            express: server,
        }
    )

    nunjucks.addGlobal("ordinal", macros.ordinal)
    nunjucks.addGlobal("time_format", macros.timeFormat)
    nunjucks.addGlobal("plural", macros.plural)
    nunjucks.addGlobal("fancy_time", macros.fancyTime)
    nunjucks.addGlobal("current_date", macros.getCurrentDate)
    nunjucks.addGlobal("truncate_string", macros.truncateString)
    nunjucks.addGlobal("mapCountryCode", macros.mapCountryCode)
    nunjucks.addGlobal("hash", "123")

    // Generate a hash of the css file
    const fileBuffer = fs.readFileSync("./assets/css/main.css");
    const hashSum = crypto.createHash("sha256");
    hashSum.update(fileBuffer);
    nunjucks.addGlobal("css_hash", hashSum.digest('hex').slice(0, 6))

    server.disable("x-powered-by")

    server.use("/assets", express.static("assets"))
    server.use(bodyParser.urlencoded({ extended: false }))

    server.use(router)
}
