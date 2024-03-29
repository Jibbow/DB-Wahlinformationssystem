import pyhdb, random, sys
from time import sleep
import unicodecsv as csv
import secrets


listStimmkreise = [209, 105, 201, 108, 124, 304, 601, 702, 121, 129, 511, 512, 501, 118, 102, 609, 704, 308, 205,
                   305, 130, 510, 605, 114, 101, 404, 306, 701, 402, 401, 307, 111, 405, 602, 110, 112, 403, 607,
                   115, 117, 303, 302, 119, 122, 406, 707, 603, 123, 508, 505, 109, 126, 710, 204, 203, 104, 127,
                   116, 128, 509, 503, 504, 712, 408, 706, 604, 125, 507, 708, 407, 703, 709, 107, 711, 207, 206,
                   713, 502, 106, 606, 506, 608, 610, 103, 301, 705, 202, 120, 113, 208, 131]
listYears = [2013, 2018]

paraSk = ""
paraYear = ""
if len(sys.argv) > 1:
    paraSk = int(sys.argv[1])
    if paraSk not in listStimmkreise:
        print("Nice try! " + paraSk)
        exit(-1)
if len(sys.argv) > 2:
    paraYear = int(sys.argv[2])
    if paraYear not in listYears:
        print("Nice try! " + paraYear)
        exit(-2)
else:
    paraYear = 2018


def createErstisForSK(y, sk):
    cursor = connection.cursor()
    sqlins = ""

    print("Starting on Stimmkreis: " + str(sk))
    sqlsel = """select *
            from wis.AGGERSTSTIMME
            where stimmkreisnr = """ + str(sk) + """
            and jahr = """ + str(y)
    cursor.execute(sqlsel)
    t = cursor.fetchall()
    test = []
    for a in t:
        test.append(list(a))
    print(test)
    sum = 0
    for row in test:
        for i in range(row[2]):
            id = row[0]
            l.append([y, id, sk])
    random.shuffle(l)


connection = pyhdb.connect(
    host=secrets.ADRESS,
    port=secrets.PORT,
    user=secrets.USERNAME,
    password=secrets.PASSWORD
)

l = []
if paraSk != "":
    createErstisForSK(paraYear, paraSk)
else:
    for sk in listStimmkreise:
        # sleep(0.5)
        createErstisForSK(paraYear, sk)

print("size: " + str(len(l)))

with open("2018erststimmeneinzeln.csv", "w+") as my_csv:
    csvWriter = csv.writer(my_csv, delimiter=';')
    csvWriter.writerows(l)