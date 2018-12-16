WITH AGGREGATPARTEISTIMMEN AS (
SELECT PARTEI, JAHR, STIMMKREIS, COUNT(*) AS ANZAHLGESAMTZWEITSTIMMEN
FROM (
    SELECT STIMMKREIS, JAHR, PARTEIID AS PARTEI
    FROM WIS.ZWEITSTIMMEPARTEI ZSP
    WHERE ZSP.JAHR={{JAHR}} AND ZSP.STIMMKREIS={{STIMMKREIS}}
    UNION ALL
    SELECT ZS.STIMMKREIS, ZS.JAHR, K.PARTEI
    FROM WIS.ZWEITSTIMMEKANDIDAT ZS JOIN WIS.KANDIDAT K ON ZS.KANDIDAT=K.ID AND ZS.JAHR=K.JAHR
    WHERE ZS.JAHR={{JAHR}} AND ZS.STIMMKREIS={{STIMMKREIS}}
) GROUP BY STIMMKREIS, JAHR, PARTEI),


/*
 * Gibt für jeden Stimmkreis die Anzahl der Zweitstimmen der Gewinnerpartei an. Dieser Wert wird dann
 * verwendet, um die tatsächliche Gewinnerpartei zu finden.
 */
MAXANZAHLZWEITSTIMMEN AS (
SELECT GES.JAHR, GES.STIMMKREIS, MAX(GES.ANZAHLGESAMTZWEITSTIMMEN) AS MAXSTIMMEN
FROM AGGREGATPARTEISTIMMEN GES
GROUP BY GES.JAHR, GES.STIMMKREIS)

SELECT P.ABKUERZUNG AS PARTEI, GES.ANZAHLGESAMTZWEITSTIMMEN AS ANZAHLZWEITSTIMMEN
FROM AGGREGATPARTEISTIMMEN GES JOIN MAXANZAHLZWEITSTIMMEN M ON GES.JAHR=M.JAHR AND GES.STIMMKREIS=M.STIMMKREIS
    JOIN WIS.PARTEI P ON GES.PARTEI=P.ID
WHERE GES.ANZAHLGESAMTZWEITSTIMMEN=M.MAXSTIMMEN
