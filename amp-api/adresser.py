import geojson
from esridump.dumper import EsriDumper

d = EsriDumper('https://services3.arcgis.com/GVgbJbqm8hXASVYi/ArcGIS/rest/services/Malmo_Sweden_Addresses/FeatureServer/0')

# Iterate over each feature
#for feature in d:
with open("adresser.geojson", "w") as f:
	geojson.dump(list(d), f)
    

