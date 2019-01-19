WITH AGGREGATKANDIDATERSTSTIMMEN AS (
SELECT K.ID AS KANDIDAT, S.JAHR AS JAHR, COUNT(*) AS ANZAHLERSTSTIMMEN, S.STIMMKREIS AS STIMMKREIS
FROM WIS.ERSTSTIMME S JOIN WIS.KANDIDAT K ON S.KANDIDAT=K.ID
WHERE S.STIMMKREIS={{STIMMKREIS}} AND S.JAHR={{JAHR}}
GROUP BY K.ID, S.JAHR, S.STIMMKREIS),


/*
 * Gibt für jede Partei die Summe aller Erststimmen für einen Stimmkreis aus.
 * Da mehrere Kandidaten in einem Stimmkreis für eine Partei antreten können, ist die Summe der Erststimmen
 * für das Ergebnis wichtig.
 */
GESAMTERSTSTIMMEN AS (
SELECT EK.JAHR, EK.STIMMKREIS, K.PARTEI, SUM(EK.ANZAHLERSTSTIMMEN) AS GESAMTERSTSTIMMEN
FROM AGGREGATKANDIDATERSTSTIMMEN EK JOIN WIS.KANDIDAT K ON EK.KANDIDAT=K.ID AND EK.JAHR=K.JAHR
    JOIN WIS.PARTEI P ON K.PARTEI=P.ID
GROUP BY EK.JAHR, EK.STIMMKREIS, K.PARTEI),

/*
 * Gibt für jeden Stimmkreis die Anzahl der Erststimmen der Gewinnerpartei an. Dieser Wert wird dann
 * verwendet, um die tatsächliche Gewinnerpartei zu finden.
 */
MAXANZAHLERSTSTIMMEN AS (
SELECT GES.JAHR, GES.STIMMKREIS, MAX(GES.GESAMTERSTSTIMMEN) AS MAXSTIMMEN
FROM GESAMTERSTSTIMMEN GES
GROUP BY GES.JAHR, GES.STIMMKREIS)

SELECT P.ABKUERZUNG AS PARTEI, P.FARBE AS PARTEI_FARBE, GES.GESAMTERSTSTIMMEN AS ANZAHLERSTSTIMMEN
FROM GESAMTERSTSTIMMEN GES JOIN MAXANZAHLERSTSTIMMEN M ON GES.JAHR=M.JAHR AND GES.STIMMKREIS=M.STIMMKREIS
    JOIN WIS.PARTEI P ON GES.PARTEI=P.ID
WHERE GES.GESAMTERSTSTIMMEN=M.MAXSTIMMEN