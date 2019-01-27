UPDATE zweitstimmekandidat
SET    kandidat = null
from kandidat, 
	 partei,
	 zweitstimmekandidat 
where kandidat.jahr = 2013 and
	zweitstimmekandidat.kandidat = kandidat.id and
	kandidat.partei = partei.id and 
	kandidat.partei = 11
	
UPDATE erststimme
SET    kandidat = null
from kandidat, 
	 partei,
	 erststimme 
where kandidat.jahr = 2013 and
	erststimme.kandidat = kandidat.id and
	kandidat.partei = partei.id and 
	kandidat.partei = 11