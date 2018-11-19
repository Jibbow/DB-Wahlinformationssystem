## Stimmengenerator
erststimmen_gen.py & zweitstimmen_gen.py

## Usage
[erst|zweit]stimmen_gen.py [sk] [year]

### Parameter
#### sk
Gibt den Stimmkreis an, für den die jewieligen Stimmen erstellt werden soll

#### year
Gibt an, ob die Daten von 2013 oder 2018 bentutz werden sollen

### Standardverhalten
Bei Weglassen der jeweiligen Paremeter wird als Jahr 2013 angenommen und der Generator für alle Stimmkreise ausgeführt.

## Algorithmus
- Berechnung der Gesamtstimmen innerhalb eines Stimmkreises
- Erstellen einer Zufallszahl
- Berechnung in welchen Bereich die Zufallszahl fällt
- Erstellen der Stimme
- Anpassen der Bereiche, so dass die Wahrscheinlichkeit der anderen Parteien steigt

## Fehlerbehandlung
 - Nicht existierende SK im Parameter werden abgefangen
 - Nicht existierende Jahre werden abgefangen

 ## Einfügen
 Jede Stimme wird einzeln per *insert into* Statement hinzugefügt, nicht besonders performant, aber realistisch xD 
