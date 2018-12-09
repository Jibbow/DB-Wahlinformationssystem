import csv


l = ['Mittelfranken.csv', 'Niederbayern.csv', 'Oberbayern.csv', 'Oberfranken.csv', 'Oberpfalz.csv', 'Schwaben.csv', 'Unterfranken.csv']

zweitstimmen = []
erststimmen = []
kandidaten = []

for e in l:
    file = open(e, "r")
    csv_reader = csv.reader(file, delimiter=";")
    header = {}
    for idx, row in enumerate(csv_reader):
        if idx == 0:
            header = row
            print header
        if idx > 0:
            wk = int(row[0])
            id = 18 * 100000 + 10000 * int(row[0]) + int(row[1]) * 100 + int(row[2])
            direkt = int(row[7]) - int(row[8])
            part = int(row[1]) + 1300
            kandidat = {'name': row[3][:20],
                        'vorname': row[4][:20],
                        'partei': part,
                        'jahr': 2018,
                        'id': id}
            kandidaten.append(kandidat)
            for index, ele in enumerate(row):
                if index > 8:
                    sk = int(header[index])
                    anz = int(ele.replace('.', ''))
                    obj = {
                        'kandidat': id,
                        'stimmkreis': sk,
                        'partei': part,
                        'stimmen': anz,
                        'jahr': 2018
                    }
                    if direkt == anz and direkt != 0:
                        erststimmen.append(obj)
                    else:
                        zweitstimmen.append(obj)

keys = kandidaten[0].keys()
with open('kandidaten.csv', 'wb') as output_file:
    dict_writer = csv.DictWriter(output_file, keys, delimiter=';', quoting=csv.QUOTE_MINIMAL)
    dict_writer.writeheader()
    dict_writer.writerows(kandidaten)


keys = erststimmen[0].keys()
with open('erststimmen.csv', 'wb') as output_file:
    dict_writer = csv.DictWriter(output_file, keys, delimiter=';', quoting=csv.QUOTE_MINIMAL)
    dict_writer.writeheader()
    dict_writer.writerows(erststimmen)


keys = zweitstimmen[0].keys()
with open('zweitstimmen.csv', 'wb') as output_file:
    dict_writer = csv.DictWriter(output_file, keys, delimiter=';', quoting=csv.QUOTE_MINIMAL)
    dict_writer.writeheader()
    dict_writer.writerows(zweitstimmen)



