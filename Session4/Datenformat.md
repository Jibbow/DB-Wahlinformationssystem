# Wahl 2013
## Baseurl
http://www.landtagswahl2013.bayern.de/

### Format der Parameter
http://www.landtagswahl2013.bayern.de/tabz19[ab][cd][e].html


#### [ab] - id des Wahlkreises - Zulässige Werte:
a = 0

b = 1..7


| Wahlkreis | Oberbayern | Niederbayern | Oberpfalz | Oberfranken | Mittelfranken | Unterfranken | Schwaben |
|-----------|------------|--------------|-----------|-------------|---------------|--------------|----------|
| ID        | 1          | 2            | 3         | 4           | 5             | 6            | 7        |
| StKr-Nr   | 01-30      | 01-09        | 01-08     | 01-08       | 01-12         | 01-10        | 01-13    |

#### [cd] - id der Partei - Zulässige Werte:

a = 0..1

b = 0..9

| Partei | CSU | SPD | FREIE WÄHLER | GRÜNE | FDP | DIE LINKE | ÖDP | REP | NPD | BP | FRAUENLISTE | PIRATEN |
|--------|-----|-----|--------------|-------|-----|-----------|-----|-----|-----|----|-------------|---------|
| ID     |  01 |  02 |      03      |   04  |  05 |     06    |  07 |  08 |  09 | 10 |      11     |    12   |

#### [e] - id der Ang. Stimmkreise - max 7 Spalten
a = (Anzahl der Stimmkreise (s. o.) - 1) / 7

## Scrapping Informationen
### Reihen die gescrappte werden müssen
/html/body/div[2]/table[1]/tbody/tr[16]

tr.content_z bis zur ersten tr.content_sz

tr.content_z, letzten 4 Ergebnisse auslassen (, dies sind die zusammengezählten Stimmen)

### innerhalb einer Zeile
#### 2.td - Name
Im Format:

Nachname, [Titel] Vorname

#### 4.td - Landtag eingezogen (erstmal unwichtig, zum testen der Implementierung wichtig)
S = Direkt gewählt im Stimmkreise

L = Wahlkreisliste

### 5.td - Stimmen gesammt
Unwichtig zum scrappen, wichtig zum verifizieren

### 6.td - Anzahl Zweitstimmen
Unwichtig zum scrappen, wichtig zum verifizieren

### 7. - letzte td
Anzahl Stimmen in dem Stimmkreis

Wenn mit Stern markiert, dann als Erststimme

## Scrapping Anmerkung
Da es nicht einheitlich viele Spalten für Stimmkreise gibt, muss man die Anzahl bzw. die ID der jeweiligen Spalten aus der ZWEITEN <tr> rausscrappen
