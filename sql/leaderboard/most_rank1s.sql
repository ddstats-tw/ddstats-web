SELECT RANK() OVER (ORDER BY COUNT(CASE WHEN rank = $1 THEN 1 END) DESC) AS "rank!",
    name,
    COUNT(CASE WHEN rank = 1 THEN 1 END) AS "rank1s!",
    COUNT(CASE WHEN rank = 2 THEN 1 END) AS "rank2s!",
    COUNT(CASE WHEN rank = 3 THEN 1 END) AS "rank3s!",
    COUNT(CASE WHEN rank = 4 THEN 1 END) AS "rank4s!",
    COUNT(CASE WHEN rank = 5 THEN 1 END) AS "rank5s!"
FROM
    rankings
JOIN maps ON
    rankings.map = maps.map 
WHERE
    rank <= 5 AND 
    maps.server LIKE CASE WHEN $2 = 'Total' THEN '%' ELSE $2 END
GROUP BY
    name
HAVING
    COUNT(CASE WHEN rank = $1 THEN 1 END) > 0
LIMIT 101 OFFSET 100*$3::smallint
