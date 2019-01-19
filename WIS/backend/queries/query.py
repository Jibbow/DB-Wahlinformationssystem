#!/bin/python

import pyhdb
import os
import sys
import csv
import argparse
from dotenv import load_dotenv

"""
The script executes a query against a database specified by environment variables.
You can either pass a query directly as an argument with the --sql flag
or specify a file which contains the query with the --file flag.

Usage:
    query.py [--sql sql_query] [--file queryfile.sql]

"""

load_dotenv()
connection = pyhdb.connect(
    host=os.getenv('DATABASE_URL'),
    port=os.getenv('DATABASE_PORT'),
    user=os.getenv('DATABASE_USER'),
    password=os.getenv('DATABASE_PASSWORD')
)


parser = argparse.ArgumentParser(description='Run a SQL query')
parser.add_argument('--file', type=str)
parser.add_argument('--sql', type=str)
args = parser.parse_args()

if args.sql is not None:
    query = args.sql
if args.file is not None:
    query = open(args.file).read()


cursor = connection.cursor()
cursor.execute(query)

# print result to stdout
result = cursor.fetchall()
csvwriter = csv.writer(sys.stdout)
csvwriter.writerows(result)
    

cursor.close()
connection.commit()
connection.close()
