import pyhdb, random, sys
import secrets

listStimmkreise = [209,105,201,108,124,304,601,
702,
121,
129,
511,
512,
501,
118,
102,
609,
704,
308,
205,
305,
130,
510,
605,
114,
101,
404,
306,
701,
402,
401,
307,
111,
405,
602,
110,
112,
403,
607,
115,
117,
303,
302,
119,
122,
406,
707,
603,
123,
508,
505,
109,
126,
710,
204,
203,
104,
127,
116,
128,
509,
503,
504,
712,
408,
706,
604,
125,
507,
708,
407,
703,
709,
107,
711,
207,
206,
713,
502,
106,
606,
506,
608,
610,
103,
301,
705,
202,
120,
113,
208]
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
    paraYear = 2013


def createErstisForSK(y, sk):
    print("Starting on Stimmkreis: " + str(sk))
    sqlsel = """select * 
            from wis.aggerststimme 
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
        sum += row[3]
    while sum > 0:
        n = random.random()
        index = 0
        partial = test[0][3]
        while partial * 1.00 / sum < n:
            index += 1
            partial += test[index][3]
        sum -= 1
        test[index][3] -= 1
        id = test[index][2]
        sqlins = """insert into "WIS"."ERSTSTIMME" (kandidat, jahr) values( """ + str(id) + "," + str(
            y) + ")"
        cursor.execute(sqlins)
        if sum % 10000 == 0:
            print("Noch " + str(sum) + " Stimmen vebrleibend.")

connection = pyhdb.connect(
    host="192.168.0.5",
    port=39015,
    user=secrets.USERNAME,
    password=secrets.PASSWORD
)
cursor = connection.cursor()

if paraSk != "":
    createErstisForSK(paraYear, paraSk)
else:
    for sk in listStimmkreise:
        createErstisForSK(paraYear, sk)

connection.commit()

connection.close()