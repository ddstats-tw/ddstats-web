SELECT rank,
    timestamp,
    id,
    players,
    time,
    map,
    server
FROM
    teamrankings
WHERE
    map = $1 AND
    rank <= $2
ORDER BY
    rank,
    players
LIMIT
    $2
