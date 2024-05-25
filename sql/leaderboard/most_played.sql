SELECT RANK() OVER (ORDER BY p.seconds DESC) AS "rank!",
    maps.server,
    p.map,
    p.seconds,
    p.mostaddicted,
    p.mostaddicted_seconds
FROM
    maps_playtime AS p
JOIN maps ON
    maps.map = p.map
WHERE
    maps.server LIKE CASE WHEN $1 = 'Total' THEN '%' ELSE $1 END
ORDER BY
    p.seconds DESC
