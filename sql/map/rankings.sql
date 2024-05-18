SELECT *
FROM
    rankings
WHERE
    map = $1 AND
    rank <= $2
ORDER BY
    rank,
    name
LIMIT
    $2
