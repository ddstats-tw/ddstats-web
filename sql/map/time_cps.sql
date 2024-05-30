SELECT * FROM (
    SELECT DISTINCT ON (name)
        name,
        cp1,
        cp2,
        cp3,
        cp4,
        cp5,
        cp6,
        cp7,
        cp8,
        cp9,
        cp10,
        cp11,
        cp12,
        cp13,
        cp14,
        cp15,
        cp16,
        cp17,
        cp18,
        cp19,
        cp20,
        cp21,
        cp22,
        cp23,
        cp24,
        cp25,
        time
    FROM
        race
    WHERE
        map = $1
    ORDER BY
        name,
        time
)
ORDER BY
    time
LIMIT
    $2
