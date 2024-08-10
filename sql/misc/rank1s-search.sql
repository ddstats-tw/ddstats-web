SELECT rank,
    rankings.server AS "server!",
    rankings.map,
    name,
    time,
    rankings.timestamp
FROM
    rankings
JOIN maps ON
    rankings.map = maps.map
WHERE
    rank = 1 AND
    maps.server != 'Fun' AND
    maps.server LIKE CASE WHEN $1 = 'All' THEN '%' ELSE $1 END
ORDER BY
    rankings.timestamp DESC
