SELECT DISTINCT ON (maps.map)
    maps AS "map!: Map",
    rankings.name,
    rankings.rank AS "rank!",
    rankings.time AS "time!",
    teamrankings.rank AS "team_rank?",
    teamrankings.time AS "team_time?"
FROM
    maps
LEFT JOIN (
    SELECT DISTINCT ON (map)
        name,
        map,
        time,
        rank
    FROM
        rankings
    WHERE
        name = $1
    ORDER BY
        map,
        rank
) AS rankings
ON
    maps.map = rankings.map
LEFT JOIN (
    SELECT DISTINCT ON (map)
        map,
        time,
        rank
    FROM
        teamrankings
    WHERE
        players @> (ARRAY[$1])::VARCHAR(16)[]
    ORDER BY
        map,
        rank
) AS teamrankings
ON
    maps.map = teamrankings.map
WHERE
    rankings.rank <= 10 OR
    teamrankings.rank <= 10
