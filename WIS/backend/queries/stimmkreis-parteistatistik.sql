
WITH PARTEISTIMMEN_HELP AS (
SELECT A.STIMMKREISNR AS STIMMKREIS, A.JAHR AS JAHR, A.PARTEIID AS PARTEI, A.ANZAHLGESAMTZWEITSTIMMEN AS ZWEITSTIMMEN, K.ID AS KANDIDAT, AK.ANZAHLERSTSTIMMEN AS ERSTSTIMMEN
FROM AGGREGATPARTEISTIMMEN A JOIN AGGREGATKANDIDATERSTSTIMMEN AK ON ((AK.STIMMKREISNR = A.STIMMKREISNR) AND (AK.JAHR = A.JAHR)) JOIN KANDIDAT K ON ((K.ID = AK.KANDIDATNR) AND (K.JAHR = AK.JAHR) AND (K.PARTEI = A.PARTEIID))),

GESAMTSTIMMEN AS (
SELECT STIMMKREIS, JAHR, (SUM(ERSTSTIMMEN) + SUM (ZWEITSTIMMEN)) AS STIMMEN
FROM PARTEISTIMMEN_HELP
GROUP BY STIMMKREIS, JAHR),

PARTEISTIMMEN AS (
SELECT STIMMKREIS, JAHR, PARTEI, (SUM(ERSTSTIMMEN) + SUM(ZWEITSTIMMEN)) AS STIMMEN
FROM PARTEISTIMMEN_HELP
GROUP BY STIMMKREIS, JAHR, PARTEI),

ERGEBNIS AS (
SELECT P.STIMMKREIS, P.JAHR, P.PARTEI, P.STIMMEN AS STIMMEN_ABSOLUT, ((P.STIMMEN * 100) / G.STIMMEN) AS STIMMEN_RELATIV
FROM PARTEISTIMMEN P JOIN GESAMTSTIMMEN G ON ((G.STIMMKREIS = P.STIMMKREIS) AND (G.JAHR = P.JAHR))),

ENTWICKLUNGEN AS (
SELECT E1.STIMMKREIS, E1.JAHR, E1.PARTEI, (E1.STIMMEN_RELATIV - E2.STIMMEN_RELATIV) AS ENTWICKLUNG
FROM ERGEBNIS E1 JOIN ERGEBNIS E2 ON ((E1.STIMMKREIS = E2.STIMMKREIS) AND (E1.JAHR = (E2.JAHR + 4)) AND (E1.PARTEI = E2.PARTEI)))

SELECT R.PARTEI, R.STIMMEN_ABSOLUT, R.STIMMEN_RELATIV, D.ENTWICKLUNG
FROM ERGEBNIS R LEFT OUTER JOIN ENTWICKLUNGEN D ON ((R.STIMMKREIS = D.STIMMKREIS) AND (R.JAHR = D.JAHR) AND (R.PARTEI = D.PARTEI))
WHERE (JAHR = 2018) AND (STIMMKREIS = 1)
