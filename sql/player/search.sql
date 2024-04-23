SELECT name,
    points,
    clan,
    country,
    skin_name,
    skin_color_body,
    skin_color_feet
FROM
    players
WHERE 
    name ILIKE FORMAT('%s%%', $1::text)
ORDER BY
    points DESC
LIMIT $2
