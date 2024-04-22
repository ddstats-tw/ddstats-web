SELECT name,
    points
FROM
    players
WHERE 
    name ILIKE FORMAT('%s%%', $1::text)
ORDER BY
    points DESC
LIMIT $2
