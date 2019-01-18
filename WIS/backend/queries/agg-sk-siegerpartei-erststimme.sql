/*
 * Gibt für jeden Stimmkreis die Anzahl der Erststimmen der Gewinnerpartei an. Dieser Wert wird dann
 * verwendet, um die tatsächliche Gewinnerpartei zu finden.
 */
with MAXANZAHLERSTSTIMMEN AS (
SELECT GES.JAHR, GES.STIMMKREIS, MAX(GES.STIMMEN) AS MAXSTIMMEN
FROM AGGREGAT_ERSTSTIMME GES JOIN WIS.KANDIDAT K ON GES.KANDIDAT=K.ID AND GES.JAHR=K.JAHR
    JOIN WIS.PARTEI P ON K.PARTEI=P.ID
GROUP BY GES.JAHR, GES.STIMMKREIS)


select abkuerzung, stimmen, m.stimmkreis, m.jahr
from AGGREGAT_ERSTSTIMME GES join MAXANZAHLERSTSTIMMEN m on m.Stimmkreis = ges.Stimmkreis 
															and m.jahr = ges.jahr
															and m.maxstimmen = ges.stimmen
							join kandidat k on k.id = ges.kandidat
							join partei p on p.id = k.partei
							where m.jahr={{JAHR}} and m.stimmkreis={{STIMMKREIS}}
