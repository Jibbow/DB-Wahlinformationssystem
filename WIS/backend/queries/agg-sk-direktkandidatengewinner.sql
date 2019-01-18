/*
 * Ermittelt die Gewinner der Erststimmen für jeden Stimmkreis.
 * Gewinner ist derjenige Kandidat, der die meisten Erststimmen erreicht hat,
 * solange die Partei, die er oder sie angehört, die 5%-Hürde geschafft hat.
 * Die 5%-Hürde errechnet sich auf Wahlkreisebene.
 * Falls die zugehörige Partei die 5%-Hürde nicht geschafft hat, gewinnt der
 * nachfolgende Kandidat.
 * ID, VORNAME, NACHNAME, PARTEI
 */


WITH AGGREGATKANDIDATERSTSTIMMEN AS (
SELECT *
FROM WIS.AGGREGAT_ERSTSTIMME es
WHERE eS.STIMMKREIS={{STIMMKREIS}} AND eS.JAHR={{JAHR}}),

AGGREGATPARTEISTIMMEN AS (
SELECT PARTEI, JAHR, STIMMKREIS, SUM(STIMMEn) AS ANZAHLGESAMTZWEITSTIMMEN
FROM (
    SELECT STIMMKREIS, JAHR, PARTEI, STIMMEn
    FROM WIS.AGGREGAT_ZWEITSTIMMEPARTEI ZSP
    WHERE ZSP.JAHR=JAHR AND ZSP.STIMMKREIS={{STIMMKREIS}}
    UNION ALL
    SELECT ZS.STIMMKREIS, ZS.JAHR, K.PARTEI, STIMMEn
    FROM WIS.AGGREGAT_ZWEITSTIMMEKANDIDAT ZS JOIN WIS.KANDIDAT K ON ZS.KANDIDAT=K.ID AND ZS.JAHR=K.JAHR
    WHERE ZS.JAHR={{JAHR}} AND ZS.STIMMKREIS={{STIMMKREIS}}
) GROUP BY STIMMKREIS, JAHR, PARTEI),


/*
 * Anzahl der abgegebenen Stimmen pro Wahlkreis und Jahr, getrennt nach Erststimmen und Zweitstimmen
 * WAHLKREIS, JAHR, ERSTSTIMMEN, ZWEITSTIMMEN
 */
WAHLKREISGESAMTSTIMMEN AS (
SELECT ES.WAHLKREIS, ES.JAHR, SUM(ES.ERSTSTIMMEN) AS ERSTSTIMMEN, SUM(ZS.ZWEITSTIMMEN) AS ZWEITSTIMMEN
FROM (
    SELECT SK.WAHLKREIS AS WAHLKREIS, SK.JAHR, E.STIMMEN AS ERSTSTIMMEN
    FROM AGGREGATKANDIDATERSTSTIMMEN E JOIN WIS.STIMMKREIS SK ON E.STIMMKREIS=SK.NR AND E.JAHR=SK.JAHR
) ES JOIN (
    SELECT WAHLKREIS, PS.JAHR AS JAHR, PS.ANZAHLGESAMTZWEITSTIMMEN AS ZWEITSTIMMEN
    FROM AGGREGATPARTEISTIMMEN PS JOIN WIS.STIMMKREIS SK ON PS.STIMMKREIS=SK.NR AND PS.JAHR=SK.JAHR
) ZS ON ES.WAHLKREIS=ZS.WAHLKREIS AND ES.JAHR=ZS.JAHR
GROUP BY ES.WAHLKREIS, ES.JAHR),

/*
 * Anzahl der Stimmen für eine Partei pro Wahlkreis pro Jahr, getrennt nach Erst- und Zweitstimmen.
 * JAHR, WAHLKREIS, PARTEI, ERSTSTIMMEN, ZWEITSTIMMEN
 */
PARTEIGESAMTSTIMMENPROWAHLKREIS AS (
SELECT SK.JAHR, SK.WAHLKREIS, K.PARTEI, ES.STIMMEN AS ERSTSTIMMEN, PS.ANZAHLGESAMTZWEITSTIMMEN AS ZWEITSTIMMEN
FROM AGGREGATPARTEISTIMMEN PS JOIN WIS.STIMMKREIS SK ON PS.JAHR=SK.JAHR AND PS.STIMMKREIS=SK.NR
    JOIN AGGREGATKANDIDATERSTSTIMMEN ES ON ES.JAHR=PS.JAHR AND ES.STIMMKREIS=PS.STIMMKREIS
    JOIN WIS.KANDIDAT K ON ES.KANDIDAT=K.ID AND ES.JAHR=K.JAHR AND PS.PARTEI=K.PARTEI),

/*
 * Parteien, die die 5%-Hürde in einem Wahlkreis geschafft haben.
 * JAHR, WAHLKREIS, PARTEI
 */
PARTEIENUEBERFUENFPROZENT AS (
SELECT PS.JAHR, PS.WAHLKREIS, PS.PARTEI
FROM WAHLKREISGESAMTSTIMMEN WKS JOIN PARTEIGESAMTSTIMMENPROWAHLKREIS PS ON WKS.JAHR=PS.JAHR AND WKS.WAHLKREIS=PS.WAHLKREIS
WHERE (PS.ERSTSTIMMEN+PS.ZWEITSTIMMEN)*100 / WKS.ERSTSTIMMEN+WKS.ZWEITSTIMMEN >= 5
),

/*
 * Filtert nur die Kandidaten aus AGGREGATKANDIDATERSTSTIMMEN heraus, deren Partei die wahlkreisweite 5%-Hürde geschafft hat.
 * JAHR, STIMMKREIS, KANDIDAT, ERSTSTIMMEN
 */
KANDIDATERGEBNISMITHUERDE AS (
SELECT KE.JAHR AS JAHR, KE.STIMMKREIS AS STIMMKREIS, KE.KANDIDAT AS KANDIDAT, KE.STIMMEN AS ERSTSTIMMEN
FROM AGGREGATKANDIDATERSTSTIMMEN KE 
    JOIN WIS.STIMMKREIS SK ON KE.STIMMKREIS=SK.NR AND KE.JAHR=SK.JAHR
    JOIN PARTEIENUEBERFUENFPROZENT PFP ON KE.JAHR=PFP.JAHR AND PFP.WAHLKREIS=SK.WAHLKREIS
    JOIN WIS.KANDIDAT K ON KE.KANDIDAT=K.ID AND K.PARTEI=PFP.PARTEI AND K.JAHR=PFP.JAHR
),

/*
 * Ermittelt die Anzahl der Stimmen für den Gewinner in einem Stimmkreis.
 * JAHR, STIMMKREIS, ANZAHLSTIMMEN
 */
STIMMKREISMAXSTIMMEN AS (
SELECT JAHR, STIMMKREIS, MAX(ERSTSTIMMEN) AS ANZAHLSTIMMEN
FROM KANDIDATERGEBNISMITHUERDE
WHERE JAHR=JAHR
GROUP BY JAHR, STIMMKREIS),

/*
 * Ermittelt die Gewinner für jeden Stimmkreis
 * JAHR, STIMMKREIS, KANDIDAT
 */
GEWINNER AS (
SELECT KE.JAHR AS JAHR, KE.STIMMKREIS AS STIMMKREIS, KE.KANDIDAT AS KANDIDAT
FROM KANDIDATERGEBNISMITHUERDE KE JOIN STIMMKREISMAXSTIMMEN MAXSTIMMEN ON KE.JAHR=MAXSTIMMEN.JAHR AND KE.STIMMKREIS=MAXSTIMMEN.STIMMKREIS
WHERE KE.ERSTSTIMMEN=MAXSTIMMEN.ANZAHLSTIMMEN
)

SELECT G.KANDIDAT AS ID, K.VORNAME AS VORNAME, K.NACHNAME AS NACHNAME, P.ABKUERZUNG AS PARTEI
FROM GEWINNER G JOIN WIS.KANDIDAT K ON G.KANDIDAT=K.ID AND G.JAHR=K.JAHR
    JOIN WIS.PARTEI P ON K.PARTEI=P.ID
