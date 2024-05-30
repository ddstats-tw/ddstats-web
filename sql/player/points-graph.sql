SELECT * FROM (
    SELECT date_series AS "date!",
        FIRST_VALUE(points) OVER (
            PARTITION BY corrected
        ) AS "points!",
        COALESCE(rankpoints, 0) AS "rank_points!",
        COALESCE(teampoints, 0) AS "team_points!"
    FROM (
        SELECT dd::DATE AS "date_series",
            historical_points.points AS "points",
            SUM(CASE WHEN historical_points.points IS NOT NULL THEN 1 END) OVER (ORDER BY dd::DATE) AS "corrected"
        FROM
            generate_series(
                (SELECT MIN(timestamp) FROM race WHERE name = $1)::TIMESTAMP,
                (CURRENT_DATE - 1)::TIMESTAMP,
                '1 day'::INTERVAL
            ) dd
        LEFT JOIN (
            SELECT map_timestamp::DATE AS "date", MAX("cumulative") AS "points" FROM (
            SELECT race.map, MIN(race.timestamp) AS "map_timestamp", (
                SUM(maps.points) OVER (ORDER BY MIN(race.timestamp))
            ) as "cumulative"
            FROM
                race
            JOIN maps ON
                race.map = maps.map
            WHERE
                name = $1
            GROUP BY
                race.map, maps.points
            ORDER BY MIN(race.timestamp)
            )
            GROUP BY map_timestamp::DATE
            ORDER BY map_timestamp::DATE
        ) AS "historical_points"
        ON historical_points.date = dd::DATE
    )
    LEFT JOIN rankedpoints ON
        rankedpoints.name = $1 AND
        rankedpoints.date = "date_series"
)
WHERE
    to_char("date!", 'FMDAY') = 'MONDAY'
