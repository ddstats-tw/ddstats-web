SELECT
    date,
    playtime.map AS "map_name!",
    name,
    SUM(time) AS "seconds_played!",
    maps AS "map?: Map"
FROM
    playtime
LEFT JOIN maps ON
    playtime.map = maps.map 
WHERE
    name = $1
GROUP BY
    name,
    date,
    playtime.map,
    maps
ORDER BY
    name,
    date DESC,
    "seconds_played!" DESC,
    playtime.map
LIMIT $2
