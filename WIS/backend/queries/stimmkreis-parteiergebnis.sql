WITH AGGREGATKANDIDATERSTSTIMMEN AS (
SELECT K.ID AS KANDIDAT, S.JAHR AS JAHR, COUNT(*) AS ANZAHLERSTSTIMMEN, S.STIMMKREIS AS STIMMKREIS
FROM WIS.ERSTSTIMME S JOIN WIS.KANDIDAT K ON S.KANDIDAT=K.ID
WHERE S.STIMMKREIS={{STIMMKREIS}} AND S.JAHR={{JAHR}}
GROUP BY K.ID, S.JAHR, S.STIMMKREIS),

AGGREGATPARTEISTIMMEN AS (
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


GESAMTSTIMMENPARTEI AS (
SELECT JAHR, STIMMKREIS, PARTEI, SUM(STIMMEN) AS STIMMEN
FROM (
    SELECT PARTEI, KS.JAHR AS JAHR, STIMMKREIS, ANZAHLERSTSTIMMEN AS STIMMEN
    FROM AGGREGATKANDIDATERSTSTIMMEN KS JOIN WIS.KANDIDAT K ON KS.JAHR=K.JAHR AND KS.KANDIDAT=K.ID
    UNION ALL
    SELECT PARTEI, JAHR, STIMMKREIS, ANZAHLGESAMTZWEITSTIMMEN AS STIMMEN
    FROM AGGREGATPARTEISTIMMEN PS
) GROUP BY PARTEI, JAHR, STIMMKREIS),

STIMMKREISGESAMTSTIMMEN AS (
SELECT JAHR, STIMMKREIS, SUM(STIMMEN) AS STIMMEN
FROM GESAMTSTIMMENPARTEI
GROUP BY JAHR, STIMMKREIS),

STATISTIK AS (
SELECT P.JAHR AS JAHR, P.STIMMKREIS AS STIMMKREIS, PARTEI, P.STIMMEN AS STIMMENABSOLUT, P.STIMMEN * 100 / GS.STIMMEN AS STIMMENRELATIV
FROM GESAMTSTIMMENPARTEI P JOIN STIMMKREISGESAMTSTIMMEN GS ON P.JAHR=GS.JAHR AND P.STIMMKREIS=GS.STIMMKREIS)

SELECT P.ABKUERZUNG AS PARTEI, STIMMENABSOLUT, STIMMENRELATIV
FROM STATISTIK S JOIN WIS.PARTEI P
ON S.PARTEI=P.ID