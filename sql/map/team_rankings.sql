SELECT *
FROM
    teamrankings
WHERE
    map = $1 AND
    rank <= $2
ORDER BY
    rank
LIMIT
    $2
