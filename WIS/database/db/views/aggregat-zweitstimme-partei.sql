CREATE VIEW WIS.AGGREGAT_ZWEITSTIMMEPARTEI (PARTEI, STIMMKREIS, JAHR, STIMMEN)
AS 
SELECT PARTEI, STIMMKREIS, JAHR, COUNT(*) AS STIMMEN
FROM WIS.ZWEITSTIMMEPARTEI
GROUP BY STIMMKREIS, JAHR, PARTEI

WITH DYNAMIC CACHE;
