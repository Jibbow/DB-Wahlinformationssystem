with stimmenProStimmkreis2018 as (
  select stimmkreis, id, jahr, count(*)
  from ((select p.id, z.stimmkreis, z.jahr
  	from (	select *
  		from zweitstimmekandidat
  		union all
  		select *
  		from erststimme) z
   	join kandidat k on z.kandidat = k.id and z.jahr = k.jahr
  	join partei p on p.id = k.partei and p.jahr = k.jahr)
  union all
  (select p.parteiid as id, p.stimmkreis, p.jahr
   	from zweitstimmepartei p))

  group by id, stimmkreis, jahr
  order by stimmkreis, id, jahr
),

/* zählen der gesammten abgegebenen Stimmen  */
gesamt2018 as (
	select *
	from 	(select count(*) as gesamterst from erststimme where jahr=2018),
			(select count(*) as gesamtzwei from zweitstimmekandidat where jahr=2018),
			(select count(*) as gesamtzweitpartei from zweitstimmepartei where jahr=2018)
    ), gesamtStimmen2018 as (
      select gesamterst + gesamtzwei + gesamtzweitpartei as summe
      from gesamt2018
    ),

/* wieviele Stimmen hat jede Partei insgesamt */
stimmenProPartei2018 as (
  select id, count(*) as anzStimmen
  from (
  		(select p.id, z.stimmkreis
  		from
  			(	select *
  				from zweitstimmekandidat
  				where jahr=2018
  				union all
  				select *
  				from erststimme
  				where jahr=2018
  			) z
   			join kandidat k on z.kandidat = k.id
  			join partei p on p.id = k.partei )
  		union all
  		(select p.parteiid as id, p.stimmkreis
   		from zweitstimmepartei p
   		where jahr=2018))
  group by id
  order by id
),


/* welche Partei hat mehr als 5% der Stimmen */
fuenfProzent2018 as (
	select spp.id
	from gesamtStimmen2018 gs, stimmenProPartei2018 spp
	where anzStimmen > 0.05*gs.summe)


/* anzahl der direktstimmen für Kandidaten über der 5% Hürde */
, direktstimmen2018 as (
	select k.id, k.nachname, es.stimmkreis, count(*) as anzStimmen, k.jahr
	from partei p join kandidat k on p.id = k.partei and k.jahr = p.jahr
		join erststimme es on es.kandidat = k.id and es.jahr = k.jahr
	where p.id in (select *
					from fuenfProzent2018)
	group by k.id, k.nachname, es.stimmkreis, k.jahr)

/* join der kandidaten und parteien, die die 5% hürde geschaffen haben */
, kandidatPartei2018 as (
	select p.id as partei, k.id as id, k.jahr
	from kandidat k join partei p on k.partei = p.id
	where p.id in (select * from fuenfprozent2018)
		and k.jahr = p.jahr)

/* anzahl der erststimmen in einem wahlkreis der partei */
, parteiErstWk2018 as(
	select wk.nr, kp.partei
	from wahlkreis wk
		join stimmkreis sk on wk.nr = sk.wahlkreis
		join erststimme es on es.stimmkreis = sk.nr and sk.jahr = es.jahr
		join kandidatPartei2018 kp on kp.id = es.kandidat and es.jahr = kp.jahr
	where sk.jahr=2018)


/* anzahl der zweitstimmen in einem wahlkreis der Partei */
, parteiZweitWk2018 as(
	select wk.nr, kp.partei
	from wahlkreis wk
		join stimmkreis sk on wk.nr = sk.wahlkreis
		join zweitstimmekandidat z on z.stimmkreis = sk.nr
		join kandidatPartei2018 kp on kp.id = z.kandidat
	where kp.jahr=sk.jahr
		and z.jahr=sk.jahr
)

/* anzahl der partei zweitstimmen in einem wahlkreis */
, parteiZweitDWk2018 as (
	select wk.nr, p.id as partei
	from wahlkreis wk
		join stimmkreis sk on wk.nr = sk.wahlkreis
		join zweitstimmepartei z on z.stimmkreis = sk.nr
		join partei p on p.id = z.parteiid
	where p.id in (select * from fuenfprozent2018)
		and p.jahr=sk.jahr
		and z.jahr=p.jahr)

/* union der stimmen in einem wahlkreis */
, parteiWk2018 as(
	select nr, partei, count(*) as stimmen
	from (	select * from parteiErstWk2018
		union all
			select * from parteiZweitWk2018
		union all
			select * from parteiZweitDWk2018)
	group by nr, partei ),
/* berechnung der gesamtstimmen in einem wahlkreis */
stimmenwk2018 as (
	select nr, sum(stimmen) as gstimmen, 2018 as jahr
	from parteiWK2018
	group by nr)
/* berechnung der max. Sitze in einem Wahlkreis (Rek gibt es in Hana nicht -.- */
, adjSitze as (
	select ws.*, wkcnt.counter + ws.sitzzahl as adjsitze
	from wahlkreissitze ws, wkcnt)
/* Berechnung der Sitze der Parteien im Wahlkreis basierend auf unterschied. Anzahl an Sitzen */
, anteilParteiWk2018 as (
	select swk.nr, pwk.partei, swk.jahr, ads.adjsitze as sitzzahl, pwk.stimmen*1.0000/swk.gstimmen as anteil,
	to_integer(ads.adjsitze * (pwk.stimmen*1.0000/swk.gstimmen)) as sitzefest,
	(ads.adjsitze * (pwk.stimmen*1.0000/swk.gstimmen)) -
		to_integer(ads.adjsitze * (pwk.stimmen*1.0000/swk.gstimmen)) as rest
	from stimmenwk2018 swk
		join parteiWk2018 pwk on swk.nr = pwk.nr
		join wahlkreis wk on wk.nr = swk.nr
		join adjsitze ads on wk.nr = ads.wahlkreis and swk.jahr=ads.jahr)
/* Berechnung der offenen PLätze für die Nachkommastelle */
, currentSitze2018 as (
	select nr as wk, sitzzahl, sum(sitzefest), sitzzahl - sum(sitzefest) as tbd
	from anteilParteiWk2018
	group by nr, sitzzahl)
/* Berechnung der Position der Nachkommastelle */
, addSitzeHelper2018 as (
		select apk1.nr as wk, apk1.partei, apk1.rest, apk1.sitzefest, cs.tbd, apk1.jahr, cs.sitzzahl,
			(select count(*) + 1
			 from anteilParteiWk2018 apk7
			 where apk7.rest > apk1.rest
			 and apk7.nr = apk1.nr
			 and apk1.sitzzahl = apk7.sitzzahl
			 and apk1.jahr = apk7.jahr) as position
		from anteilParteiWk2018 apk1
			join currentSitze2018 cs on apk1.nr = cs.wk
				and cs.sitzzahl = apk1.sitzzahl
) /* Sitz dank Nachkommastelle oder nicht */
, addSitzeWK2018 as (
	select wk, partei, jahr, sitzefest, sitzzahl,
	CASE WHEN position <= tbd  THEN 1
     								ELSE 0
 							END as addedSeat,
	sitzefest + CASE WHEN position <= tbd  THEN 1
     									ELSE 0
 							END as sitzeGes
	from addSitzeHelper2018 ash)
, direktGewinner2018 as (
	select k.partei, ds1.stimmkreis, ds1.jahr, ds1.id
	from direktstimmen2018 ds1
		join kandidat k on k.id = ds1.id and k.jahr = ds1.jahr
	where not exists (select *
					  from direktstimmen2018 ds2
					  where ds2.jahr = ds1.jahr
					  and ds2.anzStimmen > ds1.anzStimmen
					  and ds1.stimmkreis = ds2.stimmkreis))
, direktMandateWk2018 as (
	select sk.wahlkreis as wk, sk.jahr, dg.partei, count(*) as anzMandate
	from direktGewinner2018 dg
		join stimmkreis sk on sk.jahr = dg.jahr and dg.stimmkreis = sk.nr
	group by sk.wahlkreis, sk.jahr, dg.partei)
, addSitzeWk2018Erw as (
	select ask.*, dm.anzMandate
	from addSitzeWk2018 ask left outer
		join direktMandateWk2018 dm on ask.partei = dm.partei
									and ask.jahr=dm.jahr
									and ask.wk = dm.wk
), moreSitzeWk2018 as (
	select distinct  ads.*, CASE WHEN dm.anzmandate is null  THEN 0
	     								ELSE dm.anzmandate
	 							END as anzMandate
	from addSitzeWK2018 ads
	left outer join direktMandateWk2018 dm
	on ads.partei = dm.partei
	and ads.jahr = dm.jahr
	and ads.wk = dm.wk
), tmpSitze2018 as (
	select *
	from moreSitzeWk2018 ms
	where ms.sitzeges >= ms.anzmandate
		and not exists(
			select *
			from moreSitzeWk2018 ms2
			where ms.jahr = ms2.jahr
			and ms.wk = ms2.wk and ms.partei = ms2.partei
			and ms2.sitzeges >= ms2.anzmandate
			and ms2.sitzzahl < ms.sitzzahl)
), finalWkSitze2018 as (
	select wk, jahr, max(sitzzahl) as sitzzahl
	from tmpSitze2018
	group by wk, jahr
),


/* finale Kalkulation, wie oben dieses Mal wissen wir die Anzahl an Sitzen, die zur Verfügung stehen */
finalAnteilParteiWk2018 as (
	select swk.nr, pwk.partei, swk.jahr, f.sitzzahl as sitzzahl, pwk.stimmen*1.0000/swk.gstimmen as anteil,
	to_integer(f.sitzzahl * (pwk.stimmen*1.0000/swk.gstimmen)) as sitzefest,
	(f.sitzzahl * (pwk.stimmen*1.0000/swk.gstimmen)) -
		to_integer(f.sitzzahl * (pwk.stimmen*1.0000/swk.gstimmen)) as rest
	from stimmenwk2018 swk
		join parteiWk2018 pwk on swk.nr = pwk.nr
		join wahlkreis wk on wk.nr = swk.nr
		join finalWkSitze2018 f on wk.nr = f.wk and swk.jahr=f.jahr)
, finalCurrentSitze2018 as (
	select nr as wk, sitzzahl, sum(sitzefest), sitzzahl - sum(sitzefest) as tbd
	from finalAnteilParteiWk2018
	group by nr, sitzzahl)
, finaladdSitzeHelper2018 as (
		select apk1.nr as wk, apk1.partei, apk1.rest, apk1.sitzefest, cs.tbd, apk1.jahr, cs.sitzzahl,
			(select count(*) + 1
			 from finalAnteilParteiWk2018 apk7
			 where apk7.rest > apk1.rest
			 and apk7.nr = apk1.nr
			 and apk1.sitzzahl = apk7.sitzzahl
			 and apk1.jahr = apk7.jahr) as position
		from finalAnteilParteiWk2018 apk1
			join finalCurrentSitze2018 cs on apk1.nr = cs.wk
				and cs.sitzzahl = apk1.sitzzahl
), finaladdSitzeWK2018 as (
	select wk, partei, jahr, sitzefest, sitzzahl,
	CASE WHEN position <= tbd  THEN 1
     								ELSE 0
 							END as addedSeat,
	sitzefest + CASE WHEN position <= tbd  THEN 1
     									ELSE 0
 							END as sitzeGes
	from finaladdSitzeHelper2018 ash)
, finalA1 as (
	select partei, sum(sitzeges) as sitze, p.name
	from finaladdSitzeWK2018 f join partei p on f.partei = p.id
	group by partei, p.name
	order by partei
), /* Alle Kandidaten, die kein Direktmandat gewonnen haben*/
listeOhneDirekte2018 as (
  select *
  from kandidat k
  where k.id not in (select dg.id
                  from direktGewinner2018 dg)
  and jahr=2018
), /* Anz an Erst und Zweitstimmen Pro Kandidat zusammengenommen */
stimmenZusammen2018 as (
		select *
		from Erststimme
	union all
		select *
		from ZweitstimmeKandidat
), /* Zählen der Einzelstimmen */ stimmenListe2018 as (
	select distinct sk.wahlkreis, z.kandidat, count(*) as anzStimmen, sk.jahr, lod.partei
	from listeOhneDirekte2018 lod
		join stimmenZusammen2018 z on lod.id = z.kandidat and lod.jahr = z.jahr
		join stimmkreis sk on sk.nr = z.stimmkreis  and sk.jahr = z.jahr
		join wahlkreis w on w.nr = sk.wahlkreis
	group by sk.wahlkreis, z.kandidat, lod.partei, lod.id, sk.jahr
), /* Berechnung der Position auf der Liste por Partei und Wk */
posListe2018 as (
	select *, (select count(*)
				from stimmenListe2018 sl2
				where sl1.anzStimmen < sl2.anzStimmen
				and sl2.jahr = sl1.jahr
				and sl2.wahlkreis = sl1.wahlkreis
				and sl2.partei = sl1.partei) + 1 as pos
	from stimmenListe2018 sl1
),/* left Outer Join mit den DirektMandaten*/
posListeCase2018 as (
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
, /* Wieviele Mandate über die Liste */
mandatePerListe2018 as (
	select *
	from posListeCase2018 p
	where p.pos <= p.sitzeges - p.anzmandate)
, finalA2 as (
	select vorname, nachname, abkuerzung
	from (
			select m.kandidat
			from mandatePerListe2018 m
		union all
			select dg.id
		    from direktGewinner2018 dg) m
	    join kandidat k on k.id = m.kandidat and k.jahr = 2018
	    join partei p on p.id = k.partei and p.jahr = k.jahr
	    order by p.abkuerzung
), finalA5 as (
	select wk, partei, CASE WHEN (anzmandate - sitzeges)  <= 0  THEN 0
	     							  ELSE (anzmandate - sitzeges)
	 							END as uemandate,
 							w.name
	from moreSitzeWk2018 a
		join wahlkreis w on a.wk = w.nr
	where not exists
		(select *
		from moreSitzeWk2018 b
		where a.wk = b.wk
		and a.partei = b.partei
		and a.jahr = b.jahr
		and b.sitzzahl < a.sitzzahl)
	order by wk, partei)
, vergleichDirektstimmen2018 as (
	select ds1.*,
			ds3.anzStimmen as stimmen2,
			ds3.id as vKandidat,
			(select count(*)
				from direktstimmen2018 ds2
				where ds1.stimmkreis = ds2.stimmkreis
				and ds1.jahr = ds2.jahr
				and ds1.anzstimmen < ds2.anzstimmen) + 1 as posD1,
			(select count(*)
			 from direktstimmen2018 ds4
			 where ds4.stimmkreis = ds3.stimmkreis
			 and ds3.jahr = ds4.jahr
			 and ds3.anzstimmen < ds4.anzstimmen) + 1 as posD2
	from direktstimmen2018 ds1
		join direktstimmen2018 ds3
			on ds1.stimmkreis = ds3.stimmkreis
			and ds1.jahr = ds3.jahr
	order by ds1.stimmkreis, posD1)
, vergleichDirekt2018 as (
	select *, CASE WHEN (posD1 = 1 and posD2 = 2)  THEN anzstimmen - stimmen2
		     	   when (posD1 <> 1 and posD2 = 1 ) then anzstimmen - stimmen2
		     	   ELSE -9999999
		 	END as diff
	from vergleichDirektstimmen2018 v
), customSort2018 as (
	select k.id, k.jahr, diff, vKandidat, k.partei
	from vergleichDirekt2018 v
	join kandidat k on k.id = v.id and k.jahr = v.jahr
	join partei p on p.id = k.partei and p.jahr = k.jahr
	order by v.jahr, partei,  case when diff > 0 then 1
					              when diff < 0 then 2
					         end asc,
					         case when diff > 0 then diff
					         		else  			-diff
					         end asc
), gewVerSort2018 as (
	select *,
			case 	when c1.diff >= 0 then (select count(*)
												from customSort2018 c2
												where c2.diff >= 0
												and c2.diff < c1.diff
												and c2.partei = c1.partei
												and c2.jahr = c1.jahr)
			    	when diff < 0 then (select count (*)
											from customSort2018 c3
											where c3.partei = c1.partei
												and c3.jahr = c1.jahr
												and (c3.diff >= 0
												or (c3.diff < 0
													and c3.diff > c1.diff)))
			 end as pos
	from customSort2018 c1)
,  finalA6 as (
	select g.*, k.nachname, k.vorname, p.abkuerzung
	from gewVerSort2018 g
		join kandidat k on k.id = g.id and k.jahr = g.jahr
		join partei p on p.id = k.partei and p.jahr = k.jahr
	where pos < 10
	order by partei, pos
)

select *
from finalA6
