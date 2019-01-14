This directory contains everything related to the database.

The data is scraped from the internet from publicly available websites: 
 - [https://www.landtagswahl2018.bayern.de/ergebnis_stimmkreis_tabellen_101.html](https://www.landtagswahl2018.bayern.de/ergebnis_stimmkreis_tabellen_101.html)
 - [http://www.landtagswahl2013.bayern.de/taba2101.html](http://www.landtagswahl2013.bayern.de/taba2101.html)

`db-schema-definition.sql` is a SQL schema definition for SAP HANA.

`db-provisioner.py` is a little script for setting up the whole database. _Make sure to adjust the connection properties needed for connecting to the DB._ The script sets up the schema and loads the data provided in the respective directories for 2013 and 2018 into the DB.

The whole database is stored in `db.zip`. The archive is used by the `db-provisioner.py` script. Make sure that it's a zip-file containing a directory called `db`. The paths can also be adjusted in the provisioner script. The files are temporarily unpacked in a `tmp` directory next to `db.zip` and are deleted afterwards.  
When making changes to the data stored in the DB, please adjust these files accordingly. Or even better: change the contents of the files and use the DB-Provisioner script to reflect the changes in the actual database.
