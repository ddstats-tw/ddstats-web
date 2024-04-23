SELECT map,
    server,
    points,
    stars,
    mapper,
    timestamp
FROM
    maps
WHERE
    map ILIKE FORMAT('%%%s%%', $1::text)
LIMIT $2
