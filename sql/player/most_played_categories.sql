SELECT maps.server as "key!",
    SUM(time) as "seconds_played!"
FROM
    playtime
JOIN maps ON
    playtime.map = maps.map
WHERE
    name = $1
GROUP BY
    maps.server
ORDER BY
    "seconds_played!" DESC
LIMIT $2
