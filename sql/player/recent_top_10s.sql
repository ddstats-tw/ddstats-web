SELECT DISTINCT ON (recent_ranks.map, recent_ranks.timestamp)
    "rank_type!",
    recent_ranks.map AS "map!",
    time AS "time!",
    rank AS "rank!",
    recent_ranks.timestamp AS "timestamp!",
    recent_ranks.server AS "server!"
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
) AS recent_ranks
JOIN maps ON
    maps.map = recent_ranks.map
WHERE
    rank <= 10 AND
    maps.server != 'Fun'
ORDER BY
    recent_ranks.timestamp DESC,
    recent_ranks.map,
    rank
LIMIT $2
