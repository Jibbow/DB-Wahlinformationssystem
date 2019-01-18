/*
 * SQL SCHEMA definition for SAP HANA
 */


-- Idempotenz sicherstellen
DROP SCHEMA WIS CASCADE;



CREATE SCHEMA WIS;
GRANT SELECT ON SCHEMA WIS to _SYS_REPO WITH GRANT OPTION; /* Allow creating "views" on this schema */

CREATE TABLE WIS.WAHLKREIS (
	NR INT,
	NAME VARCHAR(13),
	SITZZAHL INT,
	JAHR INT,
	PRIMARY KEY (NR,JAHR));

CREATE TABLE WIS.STIMMKREIS (
	NR INT,
	JAHR INT,
	NAME NVARCHAR(47),
	STIMMBERECHTIGTE INT,
	WAHLKREIS INT,
	PRIMARY KEY (NR,JAHR),
	FOREIGN KEY (WAHLKREIS,JAHR) REFERENCES WIS.WAHLKREIS ON DELETE CASCADE);

CREATE TABLE WIS.PARTEI (
	ID INT,
	ABKUERZUNG NVARCHAR(20),
	NAME NVARCHAR(93),
	FARBE VARCHAR(6),
	PRIMARY KEY (ID));

CREATE TABLE WIS.KANDIDAT (
	ID INT,
	JAHR INT,
	VORNAME NVARCHAR(26),
	NACHNAME NVARCHAR(28),
	PARTEI INT,
	PRIMARY KEY (ID,JAHR),
	FOREIGN KEY (PARTEI) REFERENCES WIS.PARTEI);

CREATE TABLE WIS.WAHLZETTEL_ZWEITSTIMME (
	KANDIDAT INT ,
	JAHR INT,
	STIMMKREIS INT,
	KANDIDATPOS INT,
	PRIMARY KEY (KANDIDAT,STIMMKREIS,JAHR),
	FOREIGN KEY (STIMMKREIS,JAHR) REFERENCES WIS.STIMMKREIS ON DELETE CASCADE,
	FOREIGN KEY (KANDIDAT,JAHR) REFERENCES WIS.KANDIDAT ON DELETE CASCADE);

CREATE TABLE WIS.WAHLZETTEL_ERSTSTIMME (
	KANDIDAT INT,
	JAHR INT,
	STIMMKREIS INT,
	KANDIDATPOS INT,
	PRIMARY KEY (KANDIDAT,JAHR),
	FOREIGN KEY (STIMMKREIS,JAHR) REFERENCES WIS.STIMMKREIS ON DELETE CASCADE,
	FOREIGN KEY (KANDIDAT,JAHR) REFERENCES WIS.KANDIDAT ON DELETE CASCADE);

CREATE TABLE WIS.ERSTSTIMME (
	KANDIDAT INT,
	STIMMKREIS INT,
	JAHR INT,
	FOREIGN KEY (KANDIDAT,JAHR) REFERENCES WIS.KANDIDAT ON DELETE SET NULL,
	FOREIGN KEY (STIMMKREIS,JAHR) REFERENCES WIS.STIMMKREIS);

CREATE TABLE WIS.ZWEITSTIMMEPARTEI (
	PARTEI INT,
	STIMMKREIS INT,
	JAHR INT,
	FOREIGN KEY (PARTEI) REFERENCES WIS.PARTEI ON DELETE SET NULL,
	FOREIGN KEY (STIMMKREIS,JAHR) REFERENCES WIS.STIMMKREIS);

CREATE TABLE WIS.ZWEITSTIMMEKANDIDAT (
	KANDIDAT INT,
	STIMMKREIS INT,
	JAHR INT,
	FOREIGN KEY (KANDIDAT,JAHR) REFERENCES WIS.KANDIDAT ON DELETE SET NULL,
	FOREIGN KEY (STIMMKREIS,JAHR) REFERENCES WIS.STIMMKREIS);

CREATE TABLE WIS.WAHLTOKEN (
	WAHLTOKEN CHAR(36),
	JAHR INT,
	STIMMKREIS INT,
	ERSTSTIMMEABGEGEBEN BINARY(1),
	ZWEITSTIMMEABGEGEBEN BINARY(1),
	PRIMARY KEY (WAHLTOKEN,JAHR),
	FOREIGN KEY (STIMMKREIS,JAHR) REFERENCES WIS.STIMMKREIS);

/*
 * Helper table with a counter. Needed because there are no recursive queries in HANA.
 * If we manage to translate certain queries in a stored procedure instead we can 
 * dismiss the creation of this table during setup stage.
 */
CREATE TABLE WIS.WKCNT (
	COUNTER INT PRIMARY KEY)
