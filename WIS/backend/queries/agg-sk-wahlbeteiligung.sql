/*
 * Gibt die Wahlbeteiligung für einen Stimmkreis für ein Jahr zurück.
 * Die Wahlbeteiligung ist dabei nicht trivial auszurechnen, da man nicht
 * genau weiß, ob eine Person beide Stimmzettel, oder nur eine Erst- oder Zweitstimme
 * abgegeben hat. Als Annäherung nehmen wir hier den MAX Wert der jeweiligen Anzahl.
 */

WITH ANZAHLZWEITSTIMMEN AS (
SELECT STIMMKREIS, JAHR, SUM(STIMMEN) AS STIMMEN 
FROM (
    SELECT *
    FROM WIS.AGGREGAT_ZWEITSTIMMEKANDIDAT ZP
    WHERE ZP.STIMMKREIS={{STIMMKREIS}} AND ZP.JAHR={{JAHR}}
    UNION ALL
    SELECT *
    FROM AGGREGAT_ZWEITSTIMMEPARTEI ZK
    WHERE ZK.STIMMKREIS={{STIMMKREIS}} AND ZK.JAHR={{JAHR}})
GROUP BY STIMMKREIS, JAHR),

WAEHLER AS (
SELECT STIMMKREIS, JAHR, MAX(STIMMEN) AS ANZAHL
FROM (
    SELECT STIMMKREIS, JAHR, sum(STIMMEN) as stimmen FROM AGGREGAT_ERSTSTIMME
    where stimmkreis={{STIMMKREIS}} and jahr={{JAHR}}
    group by stimmkreis, jahr
    UNION ALL
    SELECT * FROM ANZAHLZWEITSTIMMEN)
GROUP BY STIMMKREIS, JAHR)

SELECT *, W.ANZAHL / SK.STIMMBERECHTIGTE * 100 AS WAHLBETEILIGUNG
FROM WIS.STIMMKREIS SK JOIN WAEHLER W ON SK.JAHR=W.JAHR AND SK.NR=W.STIMMKREIS
