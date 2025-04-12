
SELECT name,
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
    map = $1 AND
    CASE
        WHEN $2 = 'cp1' THEN cp1 != 0
        WHEN $2 = 'cp2' THEN cp2 != 0
        WHEN $2 = 'cp3' THEN cp3 != 0
        WHEN $2 = 'cp4' THEN cp4 != 0
        WHEN $2 = 'cp5' THEN cp5 != 0
        WHEN $2 = 'cp6' THEN cp6 != 0
        WHEN $2 = 'cp7' THEN cp7 != 0
        WHEN $2 = 'cp8' THEN cp8 != 0
        WHEN $2 = 'cp9' THEN cp9 != 0
        WHEN $2 = 'cp10' THEN cp10 != 0
        WHEN $2 = 'cp11' THEN cp11 != 0
        WHEN $2 = 'cp12' THEN cp12 != 0
        WHEN $2 = 'cp13' THEN cp13 != 0
        WHEN $2 = 'cp14' THEN cp14 != 0
        WHEN $2 = 'cp15' THEN cp15 != 0
        WHEN $2 = 'cp16' THEN cp16 != 0
        WHEN $2 = 'cp17' THEN cp17 != 0
        WHEN $2 = 'cp18' THEN cp18 != 0
        WHEN $2 = 'cp19' THEN cp19 != 0
        WHEN $2 = 'cp20' THEN cp20 != 0
        WHEN $2 = 'cp21' THEN cp21 != 0
        WHEN $2 = 'cp22' THEN cp22 != 0
        WHEN $2 = 'cp23' THEN cp23 != 0
        WHEN $2 = 'cp24' THEN cp24 != 0
        WHEN $2 = 'cp25' THEN cp25 != 0
        ELSE true
    END
ORDER BY
    CASE
        WHEN $2 = 'cp1' THEN cp1
        WHEN $2 = 'cp2' THEN cp2
        WHEN $2 = 'cp3' THEN cp3
        WHEN $2 = 'cp4' THEN cp4
        WHEN $2 = 'cp5' THEN cp5
        WHEN $2 = 'cp6' THEN cp6
        WHEN $2 = 'cp7' THEN cp7
        WHEN $2 = 'cp8' THEN cp8
        WHEN $2 = 'cp9' THEN cp9
        WHEN $2 = 'cp10' THEN cp10
        WHEN $2 = 'cp11' THEN cp11
        WHEN $2 = 'cp12' THEN cp12
        WHEN $2 = 'cp13' THEN cp13
        WHEN $2 = 'cp14' THEN cp14
        WHEN $2 = 'cp15' THEN cp15
        WHEN $2 = 'cp16' THEN cp16
        WHEN $2 = 'cp17' THEN cp17
        WHEN $2 = 'cp18' THEN cp18
        WHEN $2 = 'cp19' THEN cp19
        WHEN $2 = 'cp20' THEN cp20
        WHEN $2 = 'cp21' THEN cp21
        WHEN $2 = 'cp22' THEN cp22
        WHEN $2 = 'cp23' THEN cp23
        WHEN $2 = 'cp24' THEN cp24
        WHEN $2 = 'cp25' THEN cp25
        ELSE cp1
    END ASC, name
LIMIT $3
