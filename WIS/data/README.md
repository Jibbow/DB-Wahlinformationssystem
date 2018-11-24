This directory contains everything related to the database.

The data is scraped from the internet from publicly available websites. The scrapers are located in `./2018/scraper.py` (scraper for 2013 is missing).

We also want to save every single vote into the database. Therefore, we need to generate votes based on the data from the web-scraping with `vote_generator.py` files located next to the respective scraper files.

`db-schema-definition.sql` is a SQL schema definition for SAP HANA.

`db-provisioner.py` is a little script for setting up the whole database. _Make sure to adjust the connection properties needed for connecting to the DB._ The script sets up the schema and loads the data provided in the respective directories for 2013 and 2018 into the DB.
