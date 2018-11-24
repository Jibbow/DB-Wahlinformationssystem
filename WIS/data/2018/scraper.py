from lxml import html
import requests
import csv
import os

def is_number(s):
    try:
        int(s)
        return True
    except ValueError:
        return False

def remove_strings_from_list(the_list):
   return [value[:-1] for value in the_list if is_number(value)]

def remove_values_from_list(the_list, val):
   return [value for value in the_list if value != val]

def remove_star_from_list(the_list):
   return [value.replace('*','') for value in the_list]

def add_element_to_list_of_lists(the_list,val):
   return [value.insert(0,val) for value in the_list]

def remove_first_x_elements_from_list(the_list,x):
   return [value[x::] for value in the_list]

def get_wahlkreis_partys(parteiId,wahlkreisId,first):
   url = 'https://www.landtagswahl2018.bayern.de/ergebnis_einzelbewerber_90' +  str(wahlkreisId) + '_' + str(parteiId) + '_0.html#anker'
   page = requests.get(url)
   print(url)
   tree = html.fromstring(page.content)
   header = ['WahlkreisId', 'ParteiId','KandidatId','KandidatName', 'StimmzettelReihenfolge','PlatzNummer','Gesamtstimmen','Zweitstimmen']
   stimmkreise = tree.xpath('//tr[@class="tabellenkopfhoheeinzelbewerberansicht"]/th/text()')
   stimmkreise = remove_strings_from_list(stimmkreise)

   elements = tree.xpath('//td[@class = "nosilbentrennungwordbreak"]/../../tr/td/text()')
   elements = remove_values_from_list(elements,'S')
   elements = remove_values_from_list(elements,'L')
   elements = remove_star_from_list(elements)
   listlength = len(stimmkreise) + 6 
   chunks = [elements[x:x+listlength] for x in range(0, len(elements), listlength)]
   pageNumber = 0
   while True:
      pageNumber += 1
      url = 'https://www.landtagswahl2018.bayern.de/ergebnis_einzelbewerber_90' +  str(wahlkreisId) + '_' + str(parteiId) + '_' + str(pageNumber) +'.html#anker'
      page = requests.get(url)
      if page.status_code == 404:
         break
      print(url)
      tree = html.fromstring(page.content)
      stimmkreise_helper = tree.xpath('//tr[@class="tabellenkopfhoheeinzelbewerberansicht"]/th/text()')
      stimmkreise_helper = remove_strings_from_list(stimmkreise_helper)
      elements = tree.xpath('//td[@class = "nosilbentrennungwordbreak"]/../../tr/td/text()')
      elements = remove_values_from_list(elements,'S')
      elements = remove_values_from_list(elements,'L')
      elements = remove_star_from_list(elements)
      listlength = len(stimmkreise_helper) + 6 
      chunks_helper = [elements[x:x+listlength] for x in range(0, len(elements), listlength)]
      chunks_helper = remove_first_x_elements_from_list(chunks_helper,6)
      stimmkreise = stimmkreise + stimmkreise_helper

      for i in range(0,len(chunks)):
         chunks[i] = chunks[i] + chunks_helper[i]


   add_element_to_list_of_lists(chunks,parteiId)
   add_element_to_list_of_lists(chunks,wahlkreisId)
   if(first):
      chunks.insert(0,header+stimmkreise)
   return chunks
   

def get_wahlkreis_csv(wahlkreis, wahlkreisId):
   chunks = get_wahlkreis_partys(1,wahlkreisId,True)
   for i in range(2,19):
         chunks = chunks + get_wahlkreis_partys(i,wahlkreisId,False)
   kwargs = {'newline': ''}
   with open(wahlkreis+'.csv', 'w', **kwargs) as fp:
      writer = csv.writer(fp, delimiter=';')
      writer.writerows(chunks)


# create directory for storing the raw data from the website
directory = 'raw_data'
if not os.path.exists(directory):
   os.makedirs(directory)
os.chdir(directory)

# get information for every Wahlkreis and store it into a .csv file
get_wahlkreis_csv('Oberbayern',1)
get_wahlkreis_csv('Niederbayern',2)
get_wahlkreis_csv('Oberpfalz',3)
get_wahlkreis_csv('Oberfranken',4)
get_wahlkreis_csv('Mittelfranken',5)
get_wahlkreis_csv('Unterfranken',6)
get_wahlkreis_csv('Schwaben',7)

