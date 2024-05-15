SELECT gametype as "key!",
    SUM(time) as "seconds_played!"
FROM
    playtime
WHERE
    name = $1
GROUP BY
    gametype
ORDER BY
    "seconds_played!" DESC
LIMIT $2
