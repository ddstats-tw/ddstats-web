SELECT rank,
    teamrankings.map,
    players,
    time,
    teamrankings.timestamp,
    teamrankings.server as "server!"
FROM
    teamrankings
JOIN maps ON
    maps.map = teamrankings.map
WHERE
    rank = 1 AND
    maps.server != 'Fun' AND
    maps.server LIKE CASE WHEN $1 = 'All' THEN '%' ELSE $1 END
ORDER BY teamrankings.timestamp DESC
