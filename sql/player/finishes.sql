SELECT DISTINCT ON (rankings.map) 
    rankings.name,
    maps AS "map!: Map",
    rankings.timestamp,
    rankings.time,
    rankings.server,
    rankings.rank,
    "team_rank?",
    seconds_played
FROM 
    rankings
JOIN maps ON
    maps.map = rankings.map
LEFT JOIN (
    SELECT map, min(teamrankings.rank) AS "team_rank?"
    FROM 
        teamrankings
    WHERE
        teamrankings.players @> (ARRAY[$1])::VARCHAR(16)[]
    GROUP BY 
        map
) AS teamrankings ON teamrankings.map = rankings.map
LEFT JOIN (
    SELECT map, SUM(time) AS seconds_played 
    FROM
        playtime
    WHERE
        name = $1 AND
        map IN (SELECT map FROM maps)
    GROUP BY
        map
) AS playtime ON 
    rankings.map = playtime.map
WHERE 
    rankings.name = $1
ORDER BY
    rankings.map,
    rankings.time
