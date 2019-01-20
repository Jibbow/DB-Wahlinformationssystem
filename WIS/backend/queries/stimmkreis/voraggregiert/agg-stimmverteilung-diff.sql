/*
 * Berechnet das Ergebnis für die Parteien im Jahr 2013.
 * Sowohl absolute Anzahl an Stimmen als auch die relative Verteilung der Stimmen auf die Parteien.
 * Vergleiche @file/stimmkreis-parteiergebnis.sql
 */

WITH AGGREGATPARTEISTIMMEN_2013 AS (
SELECT PARTEI, JAHR, STIMMKREIS, SUM(STIMMEN) AS ANZAHLGESAMTZWEITSTIMMEN
FROM (
    SELECT STIMMKREIS, JAHR, PARTEI, STIMMEN
    FROM WIS.AGGREGAT_ZWEITSTIMMEPARTEI ZSP
    WHERE ZSP.JAHR=2013 AND ZSP.STIMMKREIS=CASE WHEN {{STIMMKREIS}}>109 AND {{STIMMKREIS}}<199 THEN {{STIMMKREIS}}-1 WHEN {{STIMMKREIS}}=109 THEN 0/*NO MATCH*/ ELSE {{STIMMKREIS}} END
    UNION ALL
    SELECT ZS.STIMMKREIS, ZS.JAHR, K.PARTEI, STIMMEN
    FROM WIS.AGGREGAT_ZWEITSTIMMEKANDIDAT ZS JOIN WIS.KANDIDAT K ON ZS.KANDIDAT=K.ID AND ZS.JAHR=K.JAHR
    WHERE ZS.JAHR=2013 AND ZS.STIMMKREIS=CASE WHEN {{STIMMKREIS}}>109 AND {{STIMMKREIS}}<199 THEN {{STIMMKREIS}}-1 WHEN {{STIMMKREIS}}=109 THEN 0/*NO MATCH*/ ELSE {{STIMMKREIS}} END
) GROUP BY STIMMKREIS, JAHR, PARTEI),


GESAMTSTIMMENPARTEI_2013 AS (
SELECT JAHR, STIMMKREIS, PARTEI, SUM(STIMMEN) AS STIMMEN
FROM (
    SELECT PARTEI, KS.JAHR AS JAHR, STIMMKREIS, STIMMEN
    FROM WIS.AGGREGAT_ERSTSTIMME KS JOIN WIS.KANDIDAT K ON KS.JAHR=K.JAHR AND KS.KANDIDAT=K.ID
    where ks.JAHR=2013 and ks.STIMMKREIS=CASE WHEN {{STIMMKREIS}}>109 AND {{STIMMKREIS}}<199 THEN {{STIMMKREIS}}-1 WHEN {{STIMMKREIS}}=109 THEN 0/*NO MATCH*/ ELSE {{STIMMKREIS}} END
    UNION ALL
    SELECT PARTEI, JAHR, STIMMKREIS, ANZAHLGESAMTZWEITSTIMMEN AS STIMMEN
    FROM AGGREGATPARTEISTIMMEN_2013 PS
) GROUP BY PARTEI, JAHR, STIMMKREIS),

STIMMKREISGESAMTSTIMMEN_2013 AS (
SELECT JAHR, STIMMKREIS, SUM(STIMMEN) AS STIMMEN
FROM GESAMTSTIMMENPARTEI_2013
GROUP BY JAHR, STIMMKREIS),

STATISTIK_2013 AS (
SELECT P.JAHR AS JAHR, P.STIMMKREIS AS STIMMKREIS, PARTEI, P.STIMMEN AS STIMMENABSOLUT, P.STIMMEN * 100 / GS.STIMMEN AS STIMMENRELATIV
FROM GESAMTSTIMMENPARTEI_2013 P JOIN STIMMKREISGESAMTSTIMMEN_2013 GS ON P.JAHR=GS.JAHR AND P.STIMMKREIS=GS.STIMMKREIS),





/*
 * Berechnet das Ergebnis für die Parteien im Jahr 2018.
 * Sowohl absolute Anzahl an Stimmen als auch die relative Verteilung der Stimmen auf die Parteien.
 * Vergleiche @file/stimmkreis-parteiergebnis.sql
 */

AGGREGATPARTEISTIMMEN_2018 AS (
SELECT PARTEI, JAHR, STIMMKREIS, SUM(STIMMEN) AS ANZAHLGESAMTZWEITSTIMMEN
FROM (
    SELECT STIMMKREIS, JAHR, PARTEI, STIMMEN
    FROM WIS.AGGREGAT_ZWEITSTIMMEPARTEI ZSP
    WHERE ZSP.JAHR=2018 AND ZSP.STIMMKREIS={{STIMMKREIS}}
    UNION ALL
    SELECT ZS.STIMMKREIS, ZS.JAHR, K.PARTEI, STIMMEN
    FROM WIS.AGGREGAT_ZWEITSTIMMEKANDIDAT ZS JOIN WIS.KANDIDAT K ON ZS.KANDIDAT=K.ID AND ZS.JAHR=K.JAHR
    WHERE ZS.JAHR=2018 AND ZS.STIMMKREIS={{STIMMKREIS}}
) GROUP BY STIMMKREIS, JAHR, PARTEI),


GESAMTSTIMMENPARTEI_2018 AS (
SELECT JAHR, STIMMKREIS, PARTEI, SUM(STIMMEN) AS STIMMEN
FROM (
    SELECT PARTEI, KS.JAHR AS JAHR, STIMMKREIS, STIMMEN
    FROM WIS.AGGREGAT_ERSTSTIMME KS JOIN WIS.KANDIDAT K ON KS.JAHR=K.JAHR AND KS.KANDIDAT=K.ID
    where ks.JAHR=2018 and ks.STIMMKREIS={{STIMMKREIS}}
    UNION ALL
    SELECT PARTEI, JAHR, STIMMKREIS, ANZAHLGESAMTZWEITSTIMMEN AS STIMMEN
    FROM AGGREGATPARTEISTIMMEN_2018 PS
) GROUP BY PARTEI, JAHR, STIMMKREIS),

STIMMKREISGESAMTSTIMMEN_2018 AS (
SELECT JAHR, STIMMKREIS, SUM(STIMMEN) AS STIMMEN
FROM GESAMTSTIMMENPARTEI_2018
GROUP BY JAHR, STIMMKREIS),

STATISTIK_2018 AS (
SELECT P.JAHR AS JAHR, P.STIMMKREIS AS STIMMKREIS, PARTEI, P.STIMMEN AS STIMMENABSOLUT, P.STIMMEN * 100 / GS.STIMMEN AS STIMMENRELATIV
FROM GESAMTSTIMMENPARTEI_2018 P JOIN STIMMKREISGESAMTSTIMMEN_2018 GS ON P.JAHR=GS.JAHR AND P.STIMMKREIS=GS.STIMMKREIS)




/*
 * Vergleicht die Ergebnisse für die Parteien aus dem Jahr 2013 mit denen aus dem Jahr 2018.
 * Parteien, die nicht mehr 2018 angetreten sind, fallen aus dem Ergebnis heraus.
 * Parteien, die nur 2018 angetreten sind, erhalten ihr Ergebnis als "Änderung".
 */
SELECT P.ABKUERZUNG AS PARTEI, P.FARBE AS PARTEI_FARBE, S18.STIMMENABSOLUT - COALESCE(S13.STIMMENABSOLUT, 0) AS DIFF_GESAMTSTIMMEN, S18.STIMMENRELATIV - COALESCE(S13.STIMMENRELATIV, 0) AS DIFF_PROZENT
FROM STATISTIK_2018 S18 LEFT OUTER JOIN STATISTIK_2013 S13 ON S18.PARTEI=S13.PARTEI
     JOIN WIS.PARTEI P ON S18.PARTEI=P.ID
ORDER BY S18.STIMMENABSOLUT DESC
