SELECT name AS "name!",
    seconds_played AS "time!"
FROM
    playtime_maps
WHERE
    map = $1
ORDER BY
    seconds_played DESC
LIMIT
    $2
