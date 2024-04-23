SELECT UNNEST(players) AS "name!",
    COUNT(*) AS "ranks_together!"
FROM
    teamrankings
WHERE
    $1 = ANY(players)
GROUP BY
    "name!"
ORDER BY
    "ranks_together!" DESC
LIMIT $2 OFFSET 1
