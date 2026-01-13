import geojson
from esridump.dumper import EsriDumper

d = EsriDumper('https://gis.malmo.se/arcgis/rest/services/FGK/Parkster/MapServer/1')

# Iterate over each feature
#for feature in d:
with open("miljöparkering.geojson", "w") as f:
	geojson.dump(list(d), f)
    

