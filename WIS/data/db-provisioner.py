import pyhdb
import csv

connection = pyhdb.connect(
    host='ec2-18-195-119-179.eu-central-1.compute.amazonaws.com',
    port='39015',
    user='SYSTEM',
    password='Jibbow123'
)
print('successfully connected to SAP HANA!')



# drop all tables (ordering is important due to 'delete cascades') and drop schema
def clean_up_db(connection):
    print('Cleaning up DB...')
    cursor = connection.cursor()
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
        print('Applying SQL schema done!')



def load_csv_file(connection, filepath, csv_delimiter, table):
    cursor = connection.cursor()
    print('Loading %s into DB...' % filepath)
    with open(filepath, newline='') as f:
        csvreader = csv.reader(f, delimiter=csv_delimiter)
        firstrow = next(csvreader, None) # skip header
        v_str = ','.join(['?' for _ in range(0, len(firstrow))]) # get number of fields

        for row in csvreader:
            cursor.execute('INSERT INTO %s VALUES (%s)' % (table, v_str), row)
    cursor.execute('SELECT COUNT(*) FROM %s' % table)
    number = cursor.fetchone()
    print('...done! (now %i entries in %s)' % (number[0], table))



def load_data(connection):
    load_csv_file(connection, './2018/Parteien.csv', ';', 'WIS.PARTEI')



# setup database:
clean_up_db(connection)
setup_schema(connection)
print('\n########################\n')
load_data(connection)

# cleanup
connection.commit()
connection.close()
print('Closed connection to SAP HANA.')

