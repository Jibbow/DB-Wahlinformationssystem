import scrapy
import math

class BlogSpider(scrapy.Spider):
 
    name = 'blogspider'
    start_urls = ['http://www.landtagswahl2013.bayern.de/tabz1905082.html']
    

   

    def parse(self, response):
        base_url = 'htte://www.landtagswahl2013.bayern.de/tabz19{:02d}{:02d}{:01d}.html'
        for v in next_url(base_url):
            print(v)

        table = response.xpath('//table[@class="content_z"]')
        header_stimmkreise = [value.xpath('string(.)').extract_first().strip().encode('ascii', 'ignore') for value in table.xpath('//th[@class="content_z_oben"]')]
        
        header_foo = ["Nummer", "Name", "Platznummer", "Gesamtstimmen", "Zweitstimmen"] + header_stimmkreise
        header_string = ','.join(header_foo)

        print(header_string)

        rows = response.xpath('//tr')
        for row in rows:
            row_string = [value.strip().encode('ascii', 'ignore') for value in row.xpath('//td//text()').extract()]
            yield {'val': row_string }




def next_url(url):
    anzahl_stimmkreise = { 1: 30, 2: 9, 3: 8, 4: 8, 5: 12, 6: 10, 7: 13 }
    mapping_wahlkreis = { 1: 'Oberbayern', 2: 'Niederbayern', 3: 'Oberpfalz', 4: 'Oberfranken', 5: 'Mittelfranken', 6: 'Unterfranken', 7: 'Schwaben'}
    mapping_partei = { 1: 'CSU', 2: 'SPD', 3: 'Freie Wahler', 4: 'Grune', 5: 'FDP', 6: 'Die Linke', 7: 'OEDP', 8: 'REP', 9: 'NPD', 10: 'BP', 11: 'Frauenliste', 12: 'Piraten' }
    for wahlkreis in range(1,8):
        for party in range(1,13):
            max_stimmkreispages = int(math.ceil(anzahl_stimmkreise[wahlkreis] / 7.0))
            for stimmkreispage in range(1, max_stimmkreispages+1):
                yield { 'wahlkreis': mapping_wahlkreis[wahlkreis], 'partei': mapping_partei[party], 'url': url.format(wahlkreis,party,stimmkreispage) }

