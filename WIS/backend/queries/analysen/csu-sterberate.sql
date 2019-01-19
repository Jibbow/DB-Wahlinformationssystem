with stimmenProStimmkreis2018 as (
  select stimmkreis, id, jahr, count(*)
  from ((select p.id, z.stimmkreis, z.jahr
  	from (	select *
  		from wis.zweitstimmekandidat
  		union all
  		select *
  		from wis.erststimme) z
   	join wis.kandidat k on z.kandidat = k.id and z.jahr = k.jahr
  	join wis.partei p on p.id = k.partei)
  union all
  (select p.partei as id, p.stimmkreis, p.jahr
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
  select id, stimmkreis, count(*) as anzStimmen
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
  		(select p.partei as id, p.stimmkreis
   		from wis.zweitstimmepartei p
   		where jahr=2018))
  group by id, stimmkreis
  order by id
), anteilProSk as (
	select *, cast(((anzStimmen*1.00) / (select sum(anzStimmen)
								from stimmenProPartei2018 s
								where s.stimmkreis = s2.stimmkreis
								group by stimmkreis) * 100) as  decimal(16,2)) as gesProzent
	from stimmenProPartei2018 s2)
,
analyseSterbeCSU as (
	select a.gesprozent as PROZENT, p.abkuerzung as PARTEI, s.sterberate as STERBERATE
	from anteilProSk a 
		join wis.partei p on a.id = p.id
		join wis.mapsklk m on a.stimmkreis = m.stimmkreis
		join wis.statistik_sterberate s on s.landkreis_plz = m.landkreis_plz
	where p.id = 1)
	
select *
from analyseSterbeCSU 
