SELECT MAX(date) AS "last_seen!", 
    name,
    clan,
    country,
    skin_name AS "skin_name!",
    skin_color_body,
    skin_color_feet,
    SUM(time) AS "seconds_played!"
FROM
    playtime
WHERE
    name = $1 AND 
    skin_name != '' 
GROUP BY
    name,
    clan,
    country,
    skin_name,
    skin_color_body,
    skin_color_feet
HAVING 
    SUM(time) > 60*60
ORDER BY
    "last_seen!" DESC
LIMIT $2
