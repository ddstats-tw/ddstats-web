SELECT SUM(time) as "total_seconds_played!",
    MIN(date) as "start_of_playtime!",
    SUM(time)/(CURRENT_DATE - MIN(date)) AS "average_seconds_played!"
FROM
    playtime
WHERE
    name = $1
HAVING
    SUM(time) > 0
