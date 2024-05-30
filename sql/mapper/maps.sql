SELECT maps AS "map!: Map",
    median_time AS "median_time?",
    finishes AS "finishes?",
    finishes_rank AS "finishes_rank?"
FROM
    maps
JOIN mapstats ON
    maps.map = mapstats.map
WHERE
    $1 = ANY (STRING_TO_ARRAY(ARRAY_TO_STRING(STRING_TO_ARRAY(mapper, ' & '), ', '), ', '))
ORDER BY
    maps.timestamp DESC
LIMIT
    $2
