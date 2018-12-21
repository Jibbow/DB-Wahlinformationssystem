with stimmenProStimmkreis2018 as (
  select stimmkreis, id, jahr, count(*)
  from ((select p.id, z.stimmkreis, z.jahr
  	from (	select *
  		from wis.zweitstimmekandidat
  		union all
  		select *
  		from wis.erststimme) z
   	join wis.kandidat k on z.kandidat = k.id and z.jahr = k.jahr
  	join wis.partei p on p.id = k.partei and p.jahr = k.jahr)
  union all
  (select p.parteiid as id, p.stimmkreis, p.jahr
   	from wis.zweitstimmepartei p))

  group by id, stimmkreis, jahr
  order by stimmkreis, id, jahr
),

/* zählen der gesammten abgegebenen Stimmen  */
gesamt2018 as (
	select *
	from 	(select count(*) as gesamterst from wis.erststimme where jahr=2018),
			(select count(*) as gesamtzwei from wis.zweitstimmekandidat where jahr=2018),
			(select count(*) as gesamtzweitpartei from wis.zweitstimmepartei where jahr=2018)
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
  				from wis.zweitstimmekandidat
  				where jahr=2018
  				union all
  				select *
  				from wis.erststimme
  				where jahr=2018
  			) z
   			join wis.kandidat k on z.kandidat = k.id
  			join wis.partei p on p.id = k.partei )
  		union all
  		(select p.parteiid as id, p.stimmkreis
   		from wis.zweitstimmepartei p
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
	from wis.partei p join wis.kandidat k on p.id = k.partei and k.jahr = p.jahr
		join wis.erststimme es on es.kandidat = k.id and es.jahr = k.jahr
	where p.id in (select *
					from fuenfProzent2018)
	group by k.id, k.nachname, es.stimmkreis, k.jahr)

, kandidatPartei2018 as (
	select p.id as partei, k.id as id, k.jahr
	from wis.kandidat k join wis.partei p on k.partei = p.id
	where p.id in (select * from fuenfprozent2018)
		and k.jahr = p.jahr)

/* anzahl der erststimmen in einem wahlkreis der partei */
, parteiErstWk2018 as(
	select wk.nr, kp.partei
	from wis.wahlkreis wk
		join wis.stimmkreis sk on wk.nr = sk.wahlkreis
		join wis.erststimme es on es.stimmkreis = sk.nr and sk.jahr = es.jahr
		join kandidatPartei2018 kp on kp.id = es.kandidat and es.jahr = kp.jahr
	where sk.jahr=2018)


/* anzahl der zweitstimmen in einem wahlkreis der Partei */
, parteiZweitWk2018 as(
	select wk.nr, kp.partei
	from wis.wahlkreis wk
		join wis.stimmkreis sk on wk.nr = sk.wahlkreis
		join wis.zweitstimmekandidat z on z.stimmkreis = sk.nr
		join kandidatPartei2018 kp on kp.id = z.kandidat
	where kp.jahr=sk.jahr
		and z.jahr=sk.jahr
)

/* anzahl der partei zweitstimmen in einem wahlkreis */
, parteiZweitDWk2018 as (
	select wk.nr, p.id as partei
	from wis.wahlkreis wk
		join wis.stimmkreis sk on wk.nr = sk.wahlkreis
		join wis.zweitstimmepartei z on z.stimmkreis = sk.nr
		join wis.partei p on p.id = z.parteiid
	where p.id in (select * from fuenfprozent2018)
		and p.jahr=sk.jahr
		and z.jahr=p.jahr)


, parteiWk2018 as(
	select nr, partei, count(*) as stimmen
	from (	select * from parteiErstWk2018
		union all
			select * from parteiZweitWk2018
		union all
			select * from parteiZweitDWk2018)
	group by nr, partei ),
stimmenwk2018 as (
	select nr, sum(stimmen) as gstimmen, 2018 as jahr
	from parteiWK2018
	group by nr)
, adjSitze as (
	select ws.*, wkcnt.counter + ws.sitzzahl as adjsitze
	from wis.wahlkreissitze ws, wis.wkcnt)
, anteilParteiWk2018 as (
	select swk.nr, pwk.partei, swk.jahr, ads.adjsitze as sitzzahl, pwk.stimmen*1.0000/swk.gstimmen as anteil,
	to_integer(ads.adjsitze * (pwk.stimmen*1.0000/swk.gstimmen)) as sitzefest,
	(ads.adjsitze * (pwk.stimmen*1.0000/swk.gstimmen)) -
		to_integer(ads.adjsitze * (pwk.stimmen*1.0000/swk.gstimmen)) as rest
	from stimmenwk2018 swk
		join parteiWk2018 pwk on swk.nr = pwk.nr
		join wis.wahlkreis wk on wk.nr = swk.nr
		join adjsitze ads on wk.nr = ads.wahlkreis and swk.jahr=ads.jahr)
, currentSitze2018 as (
	select nr as wk, sitzzahl, sum(sitzefest), sitzzahl - sum(sitzefest) as tbd
	from anteilParteiWk2018
	group by nr, sitzzahl)
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
), addSitzeWK2018 as (
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
		join wis.kandidat k on k.id = ds1.id and k.jahr = ds1.jahr
	where not exists (select *
					  from direktstimmen2018 ds2
					  where ds2.jahr = ds1.jahr
					  and ds2.anzStimmen > ds1.anzStimmen
					  and ds1.stimmkreis = ds2.stimmkreis))
, direktMandateWk2018 as (
	select sk.wahlkreis as wk, sk.jahr, dg.partei, count(*) as anzMandate
	from direktGewinner2018 dg
		join wis.stimmkreis sk on sk.jahr = dg.jahr and dg.stimmkreis = sk.nr
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


/* finale Klakulation..... */
finalAnteilParteiWk2018 as (
	select swk.nr, pwk.partei, swk.jahr, f.sitzzahl as sitzzahl, pwk.stimmen*1.0000/swk.gstimmen as anteil,
	to_integer(f.sitzzahl * (pwk.stimmen*1.0000/swk.gstimmen)) as sitzefest,
	(f.sitzzahl * (pwk.stimmen*1.0000/swk.gstimmen)) -
		to_integer(f.sitzzahl * (pwk.stimmen*1.0000/swk.gstimmen)) as rest
	from stimmenwk2018 swk
		join parteiWk2018 pwk on swk.nr = pwk.nr
		join wis.wahlkreis wk on wk.nr = swk.nr
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
	from finaladdSitzeWK2018 f join wis.partei p on f.partei = p.id
	group by partei, p.name
	order by partei
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
, finalA2 as (	
	select k.ID, VORNAME, NACHNAME, abkuerzung as PARTEI
	from (
			select m.kandidat
			from mandatePerListe2018 m
		union all
			select dg.id
		    from direktGewinner2018 dg) m
	    join wis.kandidat k on k.id = m.kandidat and k.jahr = 2018
	    join wis.partei p on p.id = k.partei and p.jahr = k.jahr
	    order by p.abkuerzung
), finalA5 as (	
	select wk, partei, CASE WHEN (anzmandate - sitzeges)  <= 0  THEN 0
	     							  ELSE (anzmandate - sitzeges) 
	 							END as uemandate, 
 							w.name
	from moreSitzeWk2018 a
		join wis.wahlkreis w on a.wk = w.nr
	where not exists 
		(select *
		from moreSitzeWk2018 b
		where a.wk = b.wk
		and a.partei = b.partei
		and a.jahr = b.jahr
		and b.sitzzahl < a.sitzzahl)
	order by wk, partei)
	
select *
from finalA2 f
