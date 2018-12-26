import pyhdb
import csv
import zipfile
import os
import shutil
from dotenv import load_dotenv

load_dotenv()

connection = pyhdb.connect(
    host=os.getenv('DATABASE_URL'),
    port=os.getenv('DATABASE_PORT'),
    user=os.getenv('DATABASE_USER'),
    password=os.getenv('DATABASE_PASSWORD')
)
print('successfully connected to SAP HANA!')



# drop all tables (ordering is important due to 'delete cascades') and drop schema
def clean_up_db(connection):
    print('Cleaning up DB...')
    cursor = connection.cursor()
    try:
        cursor.execute('DROP TABLE WIS.ZWEITSTIMMEKANDIDAT')
        cursor.execute('DROP TABLE WIS.ZWEITSTIMMEPARTEI')
        cursor.execute('DROP TABLE WIS.ERSTSTIMME')
        cursor.execute('DROP TABLE WIS.STIMMKREISLISTE')
        cursor.execute('DROP TABLE WIS.WAHLKREISLISTE')
        cursor.execute('DROP TABLE WIS.KANDIDAT')
        cursor.execute('DROP TABLE WIS.PARTEI')
        cursor.execute('DROP TABLE WIS.STIMMKREIS')
        cursor.execute('DROP TABLE WIS.WAHLKREIS')
        cursor.execute('DROP SCHEMA WIS')
    except:
        print("DROP TABLE failed (tables might not exists...)")
    cursor.close()
    print('Database cleaned up!!')



def setup_schema(connection):
    with open('db-schema-definition.sql', 'r') as schemafile:
        sqlschema=schemafile.read()
        print('Successfully read SQL SCHEMA DEFINITION')

        print('Applying schema to DB...')
        print('\n########################')

        cursor = connection.cursor()
        for command in sqlschema.split(';'):
            if not command.isspace():
                print('\n'+command.strip())
                try:
                    cursor.execute(command)
                except Exception as e:
                    print(e.args)
        cursor.close()
        print('Applying SQL schema done!')



def load_csv_file(connection, filepath, table):
    cursor = connection.cursor()
    print('Loading %s into DB...' % filepath)
    with open(filepath, newline='') as f:
        csvreader = csv.reader(f, delimiter=',')
        firstrow = next(csvreader, None) # skip header
        v_str = ','.join(['?' for _ in range(0, len(firstrow))]) # get number of fields

        data = []
        for row in csvreader:
            data.append(row)
        print('Loaded file into memory. Start uploading...')
        cursor.executemany('INSERT INTO %s VALUES (%s)' % (table, v_str), data)
    cursor.execute('SELECT COUNT(*) FROM %s' % table)
    number = cursor.fetchone()
    print('...done! (now %i entries in %s)\n' % (number[0], table))
    cursor.close()



def load_data(connection):
    load_csv_file(connection, './tmp/db/WAHLKREIS.csv', 'WIS.WAHLKREIS')
    load_csv_file(connection, './tmp/db/STIMMKREIS.csv', 'WIS.STIMMKREIS')
    load_csv_file(connection, './tmp/db/PARTEI.csv', 'WIS.PARTEI')
    load_csv_file(connection, './tmp/db/KANDIDAT.csv', 'WIS.KANDIDAT')
    load_csv_file(connection, './tmp/db/WAHLKREISLISTE.csv', 'WIS.WAHLKREISLISTE')
    load_csv_file(connection, './tmp/db/STIMMKREISLISTE.csv', 'WIS.STIMMKREISLISTE')
    load_csv_file(connection, './tmp/db/ERSTSTIMME.csv', 'WIS.ERSTSTIMME')
    load_csv_file(connection, './tmp/db/ZWEITSTIMMEPARTEI.csv', 'WIS.ZWEITSTIMMEPARTEI')
    load_csv_file(connection, './tmp/db/ZWEITSTIMMEKANDIDAT.csv', 'WIS.ZWEITSTIMMEKANDIDAT')



# setup database:
clean_up_db(connection)
setup_schema(connection)
print('\n########################\n')

with zipfile.ZipFile('db.zip', 'r') as zip_ref:
    zip_ref.extractall('./tmp/')
load_data(connection)

# cleanup
shutil.rmtree('./tmp/')
connection.commit()
connection.close()
print('Closed connection to SAP HANA.')
