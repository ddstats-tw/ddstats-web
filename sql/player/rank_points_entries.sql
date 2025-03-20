SELECT date,
    rankpoints AS "rank_points!",
    teampoints AS "team_points!"
FROM
    rankedpoints
WHERE
    name = $1 AND
    to_char(date, 'FMDAY') = 'MONDAY'
ORDER BY
    date
