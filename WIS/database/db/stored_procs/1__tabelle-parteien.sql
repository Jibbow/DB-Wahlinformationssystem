CREATE PROCEDURE WIS.GET_PARTEIEN (
OUT _out TABLE (
    ID INT,
	ABKUERZUNG NVARCHAR(20),
	NAME NVARCHAR(93),
	FARBE VARCHAR(6))) 
LANGUAGE SQLSCRIPT
SQL SECURITY INVOKER
READS SQL DATA WITH RESULT VIEW PROC_PARTEIEN AS
BEGIN

_out = SELECT ID, ABKUERZUNG, NAME, FARBE FROM WIS.PARTEI;

END;
