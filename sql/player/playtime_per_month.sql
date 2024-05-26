SELECT to_char(date::date, 'YYYY-MM') AS "year_month!",
    to_char(date::date, 'FMMonth') AS "month!",
    SUM(time) AS "seconds_played!"
FROM
    playtime
WHERE
    name = $1
GROUP BY
    "year_month!",
    "month!"
ORDER BY
    "year_month!" DESC
LIMIT $2;
