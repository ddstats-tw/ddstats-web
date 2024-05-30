SELECT DISTINCT ON (map, timestamp)
    "rank_type!",
    map AS "map!",
    time AS "time!",
    rank AS "rank!",
    timestamp AS "timestamp!",
    server AS "server!"
FROM (
    SELECT 'rank' AS "rank_type!",
        map,
        time,
        rank,
        timestamp,
        server
    FROM
        rankings
    WHERE
        name = $1
    UNION ALL
    SELECT 'team_rank' as "rank_type",
        map,
        time,
        rank,
        timestamp,
        server
    FROM teamrankings
    WHERE
        players @> (ARRAY[$1])::VARCHAR(16)[]
)
WHERE
    rank <= 10
ORDER BY
    timestamp DESC,
    map,
    rank
LIMIT $2
