CREATE PROCEDURE WIS.GET_STIMMVERTEILUNG_DIFF (IN _perform_on_aggregates BOOLEAN, IN _stimmkreis INT, 
OUT _out TABLE (
    PARTEI NVARCHAR(20),
    DIFF_GESAMTSTIMMEN INT,
    DIFF_PROZENT FLOAT
)) 
LANGUAGE SQLSCRIPT
SQL SECURITY INVOKER
READS SQL DATA WITH RESULT VIEW PROC_STIMMVERTEILUNG_DIFF AS
BEGIN

/*
 * Vergleicht die Ergebnisse für die Parteien aus dem Jahr 2013 mit denen aus dem Jahr 2018.
 * Parteien, die nicht mehr 2018 angetreten sind, fallen aus dem Ergebnis heraus.
 * Parteien, die nur 2018 angetreten sind, erhalten ihr Ergebnis als "Änderung".
 */
_out = SELECT P.ABKUERZUNG AS PARTEI, S18.GESAMTSTIMMEN - COALESCE(S13.GESAMTSTIMMEN, 0) AS DIFF_GESAMTSTIMMEN, S18.PROZENT - COALESCE(S13.PROZENT, 0) AS DIFF_PROZENT
FROM WIS.PROC_STIMMVERTEILUNG (PLACEHOLDER."$$_jahr$$" => '2018', PLACEHOLDER."$$_stimmkreis$$" => :_stimmkreis, PLACEHOLDER."$$_perform_on_aggregates$$" => :_perform_on_aggregates) S18 
    LEFT OUTER JOIN WIS.PROC_STIMMVERTEILUNG (PLACEHOLDER."$$_jahr$$" => '2013', PLACEHOLDER."$$_stimmkreis$$" => :_stimmkreis, PLACEHOLDER."$$_perform_on_aggregates$$" => :_perform_on_aggregates) S13 ON S18.PARTEI=S13.PARTEI
    JOIN WIS.PARTEI P ON S18.PARTEI=P.ID
ORDER BY S18.GESAMTSTIMMEN DESC;

END;
