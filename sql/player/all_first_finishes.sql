SELECT race.map AS "map!",
    MIN(race.timestamp) as "timestamp!",
    points as "points!"
FROM
    race
JOIN maps ON 
    race.map = maps.map
WHERE
    name = $1
GROUP BY
    race.map,
    points
ORDER BY
    MIN(race.timestamp)
