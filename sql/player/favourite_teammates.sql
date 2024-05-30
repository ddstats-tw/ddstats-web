SELECT * FROM (
    SELECT UNNEST(players) AS "name!",
        COUNT(*) AS "ranks_together!"
    FROM
        teamrankings
    WHERE
        players @> (ARRAY[$1])::VARCHAR(16)[]
    GROUP BY
        "name!"
    ORDER BY
        "ranks_together!" DESC
)
WHERE
    "name!" != $1
LIMIT $2
