SELECT maps_total.server AS "category!",
    COALESCE(player_completion.count, 0) AS "maps_finished!",
    maps_total.count AS "maps_total!"
FROM (
    SELECT server,
        COUNT(*)
    FROM
        maps
    GROUP BY
        server
) AS maps_total
LEFT JOIN (
    SELECT maps.server AS server,
        COUNT(*)
    FROM
        rankings
    JOIN maps ON 
        maps.map = rankings.map
    WHERE
        name = $1
    GROUP BY
        maps.server
) AS player_completion ON player_completion.server = maps_total.server;
