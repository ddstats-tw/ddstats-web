SELECT RANK() OVER (ORDER BY "worst!" DESC) AS "rank!",
    *
FROM (
    SELECT race.map,
        race.name,
        ROUND((race.time::numeric)/60/60, 2)::float AS "worst!"
    FROM
        race AS race
    JOIN mapinfo AS mapinfo ON
        race.map = mapinfo.map
    JOIN maps AS maps ON
        maps.map = race.map
    WHERE
        maps.server LIKE CASE WHEN $1 = 'Total' THEN '%' ELSE $1 END AND
        mapinfo.BONUS = false
    ORDER BY
        time DESC
    LIMIT 101 OFFSET 100*$2::smallint
)
