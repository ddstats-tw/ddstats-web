SELECT distinct ON (name)
    name,
    clan,
    country,
    skin_name,
    skin_color_body,
    skin_color_feet
FROM
    playtime
WHERE
    name = $1 AND
    skin_name != ''
GROUP BY name,
    clan,
    country,
    skin_name,
    skin_color_body,
    skin_color_feet
ORDER BY name,
    COUNT(*) DESC
