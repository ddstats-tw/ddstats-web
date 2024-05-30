SELECT location as "key!",
    SUM(time) as "seconds_played!"
FROM
    playtime
WHERE
    name = $1
GROUP BY
    location
ORDER BY
    "seconds_played!" DESC
LIMIT $2
