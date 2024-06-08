SELECT name,
    seconds_played
FROM
    playtime_maps
WHERE
    map = $1
ORDER BY
    seconds_played DESC
LIMIT
    $2
