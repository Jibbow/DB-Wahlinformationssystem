#!/bin/python

import subprocess

subprocess.Popen('./query.py --sql "select * from wis.wahltoken"', shell=True)
