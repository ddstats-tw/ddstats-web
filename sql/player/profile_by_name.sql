SELECT name,
    points,
    clan,
    country,
    skin_name,
    skin_color_body,
    skin_color_feet
FROM
    players
WHERE name = $1
