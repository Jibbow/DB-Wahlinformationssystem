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
  select id, stimmkreis, count(*) as anzStimmen
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
  group by id, stimmkreis
  order by id
), anteilProSk as (
	select *, cast(((anzStimmen*1.00) / (select sum(anzStimmen)
								from stimmenProPartei2018 s
								where s.stimmkreis = s2.stimmkreis
								group by stimmkreis) * 100) as  decimal(16,2)) as gesProzent
	from stimmenProPartei2018 s2)
, analyseGehaltFDP as (
	select a.gesprozent, p.abkuerzung, g.indikator
	from anteilProSk a 
		join partei p on a.id = p.id
		join mapsklk m on a.stimmkreis = m.sk
		join gehalt g on g.plz = m.lkplz
	where p.id = 1305),
analyseSterbeCSU as (
	select a.gesprozent, p.abkuerzung, s.indikator
	from anteilProSk a 
		join partei p on a.id = p.id
		join mapsklk m on a.stimmkreis = m.sk
		join sterberate s on s.plz = m.lkplz
	where p.id = 1301)
	
select *
from analyseSterbeCSU 
