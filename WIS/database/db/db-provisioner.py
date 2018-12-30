import pyhdb
import csv
import zipfile
import os
import io
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




def load_csv_file(connection, file, table):
    cursor = connection.cursor()
    csvreader = csv.reader(file, delimiter=',')
    firstrow = next(csvreader, None) # skip header
    v_str = ','.join(['?' for _ in range(0, len(firstrow))]) # get number of fields

    data = []
    for row in csvreader:
        data.append(row)
    print('Loaded file into memory. Start uploading...')
    cursor.executemany('INSERT INTO %s VALUES (%s)' % (table, v_str), data)
    
    cursor.execute('SELECT COUNT(*) FROM %s' % table)
    number = cursor.fetchone()
    print('finished! (now %i entries in %s)\n' % (number[0], table))
    cursor.close()



def setup_schemas(connection):
    print('\n##### Setting up schemas...\n')
    for file in os.listdir(os.fsencode('./schemas')):
        filename = os.fsdecode(file)
        if filename.endswith('.sql'):
            print('-> applying schema definition file: ' + filename)
            cursor = connection.cursor()
            with open('./schemas/' + filename) as sql_file:
                sql = sql_file.read()
                statements = [stmt.strip() for stmt in sql.split(';') if not stmt.isspace()]
                for stmt in statements:
                    print(stmt)
                    try:
                        cursor.execute(stmt)
                    except Exception as e:
                        print(e.args)
            cursor.close()



def setup_stored_procs(connection):
    print('\n##### Setting up stored procedures...\n')
    for file in os.listdir(os.fsencode('./stored_procs')):
        filename = os.fsdecode(file)
        if filename.endswith('.sql'):
            print('-> applying stored procedure file: ' + filename)
            cursor = connection.cursor()
            with open('./stored_procs/' + filename) as sql_file:
                sql = sql_file.read()
                try:
                    cursor.execute(sql)
                except Exception as e:
                    print(e.args)
            cursor.close()



def load_data(connection):
    print('\n##### Loading data into database...\n')
    for file in os.listdir(os.fsencode('./data')):
        filename = os.fsdecode(file)
        if filename.endswith('.csv'):
            print('-> loading file: ' + filename)
            with open('./data/' + filename) as csv_file:
                tablename = os.path.splitext(filename)[0].split('__')[1]
                load_csv_file(connection, csv_file, tablename)
        if filename.endswith('.zip'):
            print('-> loading file: ' + filename)
            with zipfile.ZipFile('./data/' + filename) as zip_file:
                with zip_file.open(os.path.splitext(filename)[0] + '.csv') as csv_binary_file:
                    csv_file  = io.TextIOWrapper(csv_binary_file)
                    tablename = os.path.splitext(filename)[0].split('__')[1]
                    load_csv_file(connection, csv_file, tablename)



## Load Schema definitions
setup_schemas(connection)

## Load Data
load_data(connection)

## Create Stored Procedures
setup_stored_procs(connection)


connection.commit()
connection.close()
