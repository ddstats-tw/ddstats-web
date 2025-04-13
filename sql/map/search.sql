SELECT maps AS "map!: Map",
    median_time AS "median_time?",
    finishes AS "finishes?",
    finishes_rank AS "finishes_rank?"
FROM
    maps
JOIN mapstats ON
    maps.map = mapstats.map
WHERE
    maps.map ILIKE FORMAT('%%%s%%', $1::text)
ORDER BY
    maps.map
LIMIT $2
