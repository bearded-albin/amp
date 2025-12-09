use geojson::FeatureReader;
//use nestify::nest;
//use serde::{Deserialize, Serialize};
//use geojson::de::from_feature;
use std::fs::read;
use std::io::BufReader;
/*
nest! {
    #[derive(Debug, Deserialize)]
    struct AdressDirty {
        r#type: String,
        geometry: #[derive(Debug, Deserialize)] struct Geometry {
            r#type: String,
            coordinates: [f64; 2],
        },
        properties: #[derive(Debug, Deserialize)] struct Properties {
            ogc_fid: usize,
            beladress: String,
            popnamn: String, //Null
            postnr: String,
            postort: String,
            adressomr: String,
            adressplat: String,
            nr_num: usize,
            nr_litt: String, //Null
            object_id: usize, //ObjectId
        },
    }
}

#[derive(Serialize)]
struct AdressClean {
    coordinates: [f64; 2],
    postnummer: String,
    adress: String,
    gata: String,
    gatunummer: String, //usize?
}
*/
fn main() {
    /*
    let feature_collection_string = r#"{
     "type": "FeatureCollection",
     "features": [
         {
           "type": "Feature",
           "geometry": { "type": "Point", "coordinates": [125.6, 10.1] },
           "properties": {
             "name": "Dinagat Islands",
             "age": 123
           }
         },
         {
           "type": "Feature",
           "geometry": { "type": "Point", "coordinates": [2.3, 4.5] },
           "properties": {
             "name": "Neverland",
             "age": 456
           }
         }
     ]
}"#
    .as_bytes();
    let io_reader = std::io::BufReader::new(feature_collection_string);
     */
    let file = read("adresser.geojson").expect("failed to read file");
    let reader = BufReader::new(file.as_slice());
    let feature_reader = FeatureReader::from_reader(reader);
    for feature in feature_reader.features() {
        let feature = feature.expect("failed to iterate over valid geojson feature");
        //println!("{:?}", feature);
        if feature.geometry.is_some()
            && feature.contains_property("postnr")
            && feature.contains_property("beladress")
            && feature.contains_property("adressomr")
            && feature.contains_property("adressplat")
        {
            let postnummer = feature
                .property("postnr")
                .expect("failed to get postnummer property")
                .as_str()//Some str conv not working
                .expect("failed to turn postnummer to &str")
                .to_string();
            let adress = feature
                .property("beladress")
                .expect("failed to get adress property")
                .as_str()
                .expect("failed to turn adress to &str")
                .to_string();
            let gata = feature
                .property("adressomr")
                .expect("failed to get gata property")
                .as_str()
                .expect("failed to turn gata to &str")
                .to_string();
            let gatunummer = feature
                .property("adressplat")
                .expect("failed to get gatunummer property")
                .as_str()
                .expect("failed to turn gatunummer to &str")
                .to_string();
            let c = feature
                .geometry.unwrap().value; //Extract coords
            //let coordinates = c;
            println!("{:?}", c);
            println!("{}", postnummer);
            println!("{}", adress);
            println!("{}", gata);
            println!("{}", gatunummer);
        }
    }
}
