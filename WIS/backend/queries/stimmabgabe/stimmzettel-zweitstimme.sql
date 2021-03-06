SELECT P.NAME AS PARTEI, P.ABKUERZUNG AS PARTEI_ABKUERZUNG, P.ID AS PARTEI_ID, K.VORNAME AS KANDIDAT_VORNAME, K.NACHNAME AS KANDIDAT_NACHNAME, K.ID AS KANDIDAT_ID, WZZ.KANDIDATPOS AS LISTENPOSITION
FROM WIS.WAHLZETTEL_ZWEITSTIMME WZZ JOIN WIS.KANDIDAT K ON WZZ.KANDIDAT=K.ID AND WZZ.JAHR=K.JAHR
    JOIN WIS.PARTEI P ON K.PARTEI=P.ID
WHERE WZZ.STIMMKREIS=? AND WZZ.JAHR=?
ORDER BY P.ID, WZZ.KANDIDATPOS
