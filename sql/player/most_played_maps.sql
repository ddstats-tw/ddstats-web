SELECT playtime.map AS "map_name!",
    SUM(time) AS "seconds_played!",
    maps AS "map?: Map"
FROM 
    playtime
LEFT JOIN maps ON 
    maps.map = playtime.map
WHERE 
    name = $1
GROUP BY
    "map_name!",
    maps
ORDER BY
    "seconds_played!" DESC 
LIMIT $2
