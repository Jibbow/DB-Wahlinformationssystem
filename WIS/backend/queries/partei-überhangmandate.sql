 WITH
     /* zählen der gesammten abgegebenen Stimmen  */
     GESAMT2018 AS (SELECT *
           FROM (SELECT Count(*) AS gesamterst
                   FROM WIS.ERSTSTIMME
                  WHERE JAHR = 2018),(SELECT Count(*) AS gesamtzwei
                   FROM WIS.ZWEITSTIMMEKANDIDAT
                  WHERE JAHR = 2018),(SELECT Count(*) AS gesamtzweitpartei
                   FROM WIS.ZWEITSTIMMEPARTEI
                  WHERE JAHR = 2018)),
     GESAMTSTIMMEN2018 AS (SELECT GESAMTERST + GESAMTZWEI + GESAMTZWEITPARTEI AS summe
           FROM GESAMT2018),
     /* wieviele Stimmen hat jede Partei insgesamt */
     STIMMENPROPARTEI2018 AS (SELECT ID,Count(*) AS anzStimmen
           FROM ((SELECT P.ID,Z.STIMMKREIS
                    FROM (SELECT *
                            FROM WIS.ZWEITSTIMMEKANDIDAT
                           WHERE JAHR = 2018
                          UNION ALL
                          SELECT *
                            FROM WIS.ERSTSTIMME
                           WHERE JAHR = 2018) Z
                         JOIN WIS.KANDIDAT K
                           ON Z.KANDIDAT = K.ID
                         JOIN WIS.PARTEI P
                           ON P.ID = K.PARTEI)
                 UNION ALL
                 (SELECT P.PARTEI AS id,P.STIMMKREIS
                    FROM WIS.ZWEITSTIMMEPARTEI P
                   WHERE JAHR = 2018))
          GROUP BY ID
          ORDER BY ID),
     /* welche Partei hat mehr als 5% der Stimmen */
     FUENFPROZENT2018 AS (SELECT SPP.ID
           FROM GESAMTSTIMMEN2018 GS,STIMMENPROPARTEI2018 SPP
          WHERE ANZSTIMMEN > 0.05 * GS.SUMME)
/* anzahl der direktstimmen für Kandidaten über der 5% Hürde */
,
     DIREKTSTIMMEN2018 AS (SELECT K.ID,K.NACHNAME,ES.STIMMKREIS,Count(*) AS anzStimmen,K.JAHR
           FROM WIS.PARTEI P
                JOIN WIS.KANDIDAT K
                  ON P.ID = K.PARTEI
                JOIN WIS.ERSTSTIMME ES
                  ON ES.KANDIDAT = K.ID
                     AND ES.JAHR = K.JAHR
          WHERE K.JAHR=2018 AND P.ID IN (SELECT *
                           FROM FUENFPROZENT2018)
          GROUP BY K.ID,K.NACHNAME,ES.STIMMKREIS,K.JAHR),
     KANDIDATPARTEI2018 AS (SELECT P.ID AS partei,K.ID AS id,K.JAHR
           FROM WIS.KANDIDAT K
                JOIN WIS.PARTEI P
                  ON K.PARTEI = P.ID
          WHERE K.JAHR=2018 AND P.ID IN (SELECT *
                           FROM FUENFPROZENT2018))
/* anzahl der erststimmen in einem wahlkreis der partei */
,
     PARTEIERSTWK2018 AS (SELECT WK.NR,KP.PARTEI
           FROM WIS.WAHLKREIS WK
                JOIN WIS.STIMMKREIS SK
                  ON WK.NR = SK.WAHLKREIS AND WK.JAHR=WK.JAHR
                JOIN WIS.ERSTSTIMME ES
                  ON ES.STIMMKREIS = SK.NR
                     AND SK.JAHR = ES.JAHR
                JOIN KANDIDATPARTEI2018 KP
                  ON KP.ID = ES.KANDIDAT
                     AND ES.JAHR = KP.JAHR
          WHERE WK.JAHR = 2018)
/* anzahl der zweitstimmen in einem wahlkreis der Partei */
,
     PARTEIZWEITWK2018 AS (SELECT WK.NR,KP.PARTEI
           FROM WIS.WAHLKREIS WK
                JOIN WIS.STIMMKREIS SK
                  ON WK.NR = SK.WAHLKREIS AND WK.JAHR=SK.JAHR
                JOIN WIS.ZWEITSTIMMEKANDIDAT Z
                  ON Z.STIMMKREIS = SK.NR AND SK.JAHR=Z.JAHR
                JOIN KANDIDATPARTEI2018 KP
                  ON KP.ID = Z.KANDIDAT AND KP.JAHR=Z.JAHR)
/* anzahl der partei zweitstimmen in einem wahlkreis */
,
     PARTEIZWEITDWK2018 AS (SELECT WK.NR,P.ID AS partei
           FROM WIS.WAHLKREIS WK
                JOIN WIS.STIMMKREIS SK
                  ON WK.NR = SK.WAHLKREIS AND WK.JAHR=SK.JAHR
                JOIN WIS.ZWEITSTIMMEPARTEI Z
                  ON Z.STIMMKREIS = SK.NR AND Z.JAHR=SK.JAHR
                JOIN WIS.PARTEI P
                  ON P.ID = Z.PARTEI
          WHERE Z.JAHR=2018 AND P.ID IN (SELECT *
                           FROM FUENFPROZENT2018))
/* dont question the /2 ... fix it
TODO: fix the /2
*/
,
     PARTEIWK2018 AS (SELECT NR,PARTEI,Count(*) AS stimmen
           FROM (SELECT *
                   FROM PARTEIERSTWK2018
                 UNION ALL
                 SELECT *
                   FROM PARTEIZWEITWK2018
                 UNION ALL
                 SELECT *
                   FROM PARTEIZWEITDWK2018)
          GROUP BY NR,PARTEI),
     STIMMENWK2018
     AS (SELECT NR,Sum(STIMMEN) AS gstimmen,2018 AS jahr
           FROM PARTEIWK2018
          GROUP BY NR),
     ADJSITZE
     AS (SELECT WS.*,WKCNT.COUNTER + WS.SITZZAHL AS adjsitze
           FROM (select nr as wahlkreis, sitzzahl, jahr from wis.wahlkreis) WS,WIS.WKCNT),
     ANTEILPARTEIWK2018 AS (SELECT SWK.NR,PWK.PARTEI,SWK.JAHR,ADS.ADJSITZE AS sitzzahl,
                   PWK.STIMMEN * 1.0000 / SWK.GSTIMMEN AS anteil,
                TO_INTEGER(ADS.ADJSITZE *
                           (
                           PWK.STIMMEN * 1.0000 / SWK.GSTIMMEN )) AS
                sitzefest,
                           ( ADS.ADJSITZE * (
                             PWK.STIMMEN * 1.0000 / SWK.GSTIMMEN ) )
                           -
                TO_INTEGER(ADS.ADJSITZE
                * ( PWK.STIMMEN * 1.0000 / SWK.GSTIMMEN )) AS rest
           FROM STIMMENWK2018 SWK
                JOIN PARTEIWK2018 PWK
                  ON SWK.NR = PWK.NR
                JOIN WIS.WAHLKREIS WK
                  ON WK.NR = SWK.NR AND WK.JAHR=SWK.JAHR
                JOIN ADJSITZE ADS
                  ON WK.NR = ADS.WAHLKREIS
                     AND SWK.JAHR = ADS.JAHR),
     CURRENTSITZE2018 AS (SELECT NR AS wk,SITZZAHL,
                SITZZAHL - Sum(SITZEFEST) AS tbd
           FROM ANTEILPARTEIWK2018
          GROUP BY NR,SITZZAHL),
     ADDSITZEHELPER2018 AS (SELECT APK1.NR AS wk,APK1.PARTEI,APK1.REST,APK1.SITZEFEST,CS.TBD,
                APK1.JAHR,
                CS.SITZZAHL,
                   (SELECT Count(*) + 1
                           FROM ANTEILPARTEIWK2018 APK7
                           WHERE APK7.REST > APK1.REST
                                 AND APK7.NR = APK1.NR
                                 AND APK1.SITZZAHL = APK7.SITZZAHL
                                 AND APK1.JAHR = APK7.JAHR) AS position
           FROM ANTEILPARTEIWK2018 APK1
                JOIN CURRENTSITZE2018 CS
                  ON APK1.NR = CS.WK
                     AND CS.SITZZAHL = APK1.SITZZAHL),
     ADDSITZEWK2018 AS (SELECT WK,PARTEI,JAHR,SITZEFEST,SITZZAHL,CASE
                                                    WHEN POSITION <= TBD THEN 1
                                                    ELSE 0
                                                  END AS addedSeat,
                   SITZEFEST + CASE WHEN POSITION <= TBD THEN 1 ELSE 0 END AS
                   sitzeGes
           FROM ADDSITZEHELPER2018 ASH),
     DIREKTGEWINNER2018 AS (SELECT K.PARTEI,DS1.STIMMKREIS,DS1.JAHR,DS1.ID
           FROM DIREKTSTIMMEN2018 DS1
                JOIN WIS.KANDIDAT K
                  ON K.ID = DS1.ID
                     AND K.JAHR = DS1.JAHR
          WHERE NOT EXISTS (SELECT *
                              FROM DIREKTSTIMMEN2018 DS2
                             WHERE DS2.JAHR = DS1.JAHR
                                   AND DS2.ANZSTIMMEN > DS1.ANZSTIMMEN
                                   AND DS1.STIMMKREIS = DS2.STIMMKREIS)),
     DIREKTMANDATEWK2018 AS (SELECT SK.WAHLKREIS AS wk,SK.JAHR,DG.PARTEI,Count(*) AS anzMandate
           FROM DIREKTGEWINNER2018 DG
                JOIN WIS.STIMMKREIS SK
                  ON SK.JAHR = DG.JAHR
                     AND DG.STIMMKREIS = SK.NR
          GROUP BY SK.WAHLKREIS,SK.JAHR,DG.PARTEI),
     MORESITZEWK2018 AS (SELECT DISTINCT ADS.*,CASE
                                 WHEN DM.ANZMANDATE IS NULL THEN 0
                                 ELSE DM.ANZMANDATE
                               END AS anzMandate
           FROM ADDSITZEWK2018 ADS
                LEFT OUTER JOIN DIREKTMANDATEWK2018 DM
                             ON ADS.PARTEI = DM.PARTEI
                                AND ADS.JAHR = DM.JAHR
                                AND ADS.WK = DM.WK),
     TMPSITZE2018 AS (SELECT *
           FROM MORESITZEWK2018 MS
          WHERE MS.SITZEGES >= MS.ANZMANDATE
                AND NOT EXISTS(SELECT *
                                 FROM MORESITZEWK2018 MS2
                                WHERE MS.JAHR = MS2.JAHR
                                      AND MS.WK = MS2.WK
                                      AND MS.PARTEI = MS2.PARTEI
                                      AND MS2.SITZEGES >= MS2.ANZMANDATE
                                      AND MS2.SITZZAHL < MS.SITZZAHL)),
     FINALWKSITZE2018 AS (SELECT WK,JAHR,Max(SITZZAHL) AS sitzzahl
           FROM TMPSITZE2018
          GROUP BY WK,JAHR),
     /* finale Klakulation..... */
     FINALANTEILPARTEIWK2018 AS (SELECT SWK.NR,PWK.PARTEI,SWK.JAHR,F.SITZZAHL AS sitzzahl,
                   PWK.STIMMEN * 1.0000 / SWK.GSTIMMEN AS anteil,
                TO_INTEGER(F.SITZZAHL *
                           (
                           PWK.STIMMEN * 1.0000 / SWK.GSTIMMEN )) AS
                sitzefest,(
                   F.SITZZAHL * ( PWK.STIMMEN * 1.0000 / SWK.GSTIMMEN ) ) -
                   TO_INTEGER(F.SITZZAHL
                *
                ( PWK.STIMMEN * 1.0000 / SWK.GSTIMMEN )) AS rest
           FROM STIMMENWK2018 SWK
                JOIN PARTEIWK2018 PWK
                  ON SWK.NR = PWK.NR
                JOIN WIS.WAHLKREIS WK
                  ON WK.NR = SWK.NR AND WK.JAHR=SWK.JAHR
                JOIN FINALWKSITZE2018 F
                  ON WK.NR = F.WK
                     AND SWK.JAHR = F.JAHR),
     FINALCURRENTSITZE2018 AS (SELECT NR AS wk,SITZZAHL,Sum(SITZEFEST),
                SITZZAHL - Sum(SITZEFEST) AS tbd
           FROM FINALANTEILPARTEIWK2018
          GROUP BY NR,SITZZAHL),
     FINALADDSITZEHELPER2018 AS (SELECT APK1.NR AS wk,APK1.PARTEI,APK1.REST,APK1.SITZEFEST,CS.TBD,
                APK1.JAHR,
                CS.SITZZAHL,
                   (SELECT Count(*) + 1
                           FROM FINALANTEILPARTEIWK2018 APK7
                           WHERE APK7.REST > APK1.REST
                                 AND APK7.NR = APK1.NR
                                 AND APK1.SITZZAHL = APK7.SITZZAHL
                                 AND APK1.JAHR = APK7.JAHR) AS position
           FROM FINALANTEILPARTEIWK2018 APK1
                JOIN FINALCURRENTSITZE2018 CS
                  ON APK1.NR = CS.WK
                     AND CS.SITZZAHL = APK1.SITZZAHL),
     FINALADDSITZEWK2018 AS (SELECT WK,PARTEI,JAHR,SITZEFEST,SITZZAHL,CASE
                                                    WHEN POSITION <= TBD THEN 1
                                                    ELSE 0
                                                  END AS addedSeat,
                   SITZEFEST + CASE WHEN POSITION <= TBD THEN 1 ELSE 0 END AS
                   sitzeGes
           FROM FINALADDSITZEHELPER2018 ASH),

finalA1 as (SELECT P.NAME AS PARTEI, SUM(SITZEGES) AS SITZE
  FROM FINALADDSITZEWK2018 F
       JOIN WIS.PARTEI P
         ON F.PARTEI = P.ID
 GROUP BY PARTEI,P.NAME
 ORDER BY SUM(SITZEGES) DESC
), listeOhneDirekte2018 as (
  select *
  from wis.kandidat k
  where k.id not in (select dg.id
                  from direktGewinner2018 dg)
  and jahr=2018
), stimmenZusammen2018 as (
		select *
		from wis.Erststimme
	union all
		select *
		from wis.ZweitstimmeKandidat
), stimmenListe2018 as (
	select distinct sk.wahlkreis, z.kandidat, count(*) as anzStimmen, sk.jahr, lod.partei
	from listeOhneDirekte2018 lod
		join stimmenZusammen2018 z on lod.id = z.kandidat and lod.jahr = z.jahr
		join wis.stimmkreis sk on sk.nr = z.stimmkreis  and sk.jahr = z.jahr
		join wis.wahlkreis w on w.nr = sk.wahlkreis
	group by sk.wahlkreis, z.kandidat, lod.partei, lod.id, sk.jahr
), posListe2018 as (
	select *, (select count(*)
				from stimmenListe2018 sl2
				where sl1.anzStimmen < sl2.anzStimmen
				and sl2.jahr = sl1.jahr
				and sl2.wahlkreis = sl1.wahlkreis
				and sl2.partei = sl1.partei) + 1 as pos
	from stimmenListe2018 sl1
), posListeCase2018 as (
	select distinct wahlkreis, p.partei, pos, kandidat,
		CASE WHEN anzMandate is null  THEN 0
	     							  ELSE anzMandate
	 							END as anzMandate,
	 				f.sitzeges
	from posListe2018 p
		join finaladdSitzeWK2018 f
			on p.partei = f.partei
			and p.jahr = f.jahr
			and p.partei = f.partei
			and f.wk = p.wahlkreis
		left outer join direktMandateWk2018 dm
			on f.partei = dm.partei
			and f.jahr = dm.jahr
			and f.wk = dm.wk)
, mandatePerListe2018 as (
	select *
	from posListeCase2018 p
	where p.pos <= p.sitzeges - p.anzmandate)
, finalA5 as (
	select wk as WAHLKREISID, p.id as PARTEI, CASE WHEN (anzmandate - sitzeges)  <= 0  THEN 0
	     							  ELSE (anzmandate - sitzeges)
	 							END as UEBERHANGMANDATE,
 							w.name as WAHLKREIS,
							 a.jahr as JAHR
	from moreSitzeWk2018 a
		join wis.wahlkreis w on a.wk = w.nr and w.jahr=a.jahr
		join wis.partei p on p.id=a.partei
	where not exists
		(select *
		from moreSitzeWk2018 b
		where a.wk = b.wk
		and a.partei = b.partei
		and a.jahr = b.jahr
		and b.sitzzahl < a.sitzzahl)
	order by wk, p.abkuerzung)

select UEBERHANGMANDATE, WAHLKREIS, abkuerzung as partei
from finalA5 f
    join WIS.partei p on P.id = f.partei
where wahlkreisid=? and partei=? and jahr=?
