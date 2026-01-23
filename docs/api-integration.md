# API Integration

AMP fetches geospatial data from Malmö's Open Data Portal using ESRI ArcGIS Feature Services.

## Data Sources

### 1. Miljöparkering (Environmental Parking)

**URL:** `https://opendata.malmo.se/@fastighets-och-gatukontoret/miljoparkering/73490f00-0d71-4b17-903c-f77ab7664a53`

**Format:** GeoJSON FeatureCollection

**Fields:**
- `geometry` — LineString (parking zone boundaries)
- `properties.INFO` — Restriction description
- `properties.TID` — Time restrictions
- `properties.DAG` — Day of week (bitmask)

**Example:**
```json
{
  "type": "Feature",
  "geometry": {
    "type": "LineString",
    "coordinates": [[13.0024, 55.6050], [13.0028, 55.6052]]
  },
  "properties": {
    "INFO": "Miljözon",
    "TID": "06:00-18:00",
    "DAG": 31
  }
}
```

### 2. Parkeringsavgifter (Parking Fees)

**URL:** `https://opendata.malmo.se/@fastighets-och-gatukontoret/parkeringsavgifter/1a6bd68b-30ca-40a5-9d62-01e2a566982e`

**Format:** GeoJSON FeatureCollection

**Fields:**
- `geometry` — LineString (parking zone boundaries)
- `properties.BESKRIVNING` — Fee description
- `properties.TID` — Time restrictions
- `properties.VECKODAG` — Weekday restrictions

### 3. Adresser (Addresses)

**URL:** `https://opendata.malmo.se/@stadsbyggnadskontoret/adresser/caf1cee8-9af2-4a75-8fb7-f1d7cb11daeb`

**Format:** GeoJSON FeatureCollection

**Fields:**
- `geometry` — Point (address coordinates)
- `properties.ADRESS` — Full address string
- `properties.GATA` — Street name
- `properties.GATUNUMMER` — Street number
- `properties.POSTNUMMER` — Postal code

**Example:**
```json
{
  "type": "Feature",
  "geometry": {
    "type": "Point",
    "coordinates": [13.0024, 55.6050]
  },
  "properties": {
    "ADRESS": "Stortorget 1",
    "GATA": "Stortorget",
    "GATUNUMMER": "1",
    "POSTNUMMER": "211 22"
  }
}
```

## Implementation

**Module:** `core/src/api.rs`

### Main Function

```rust
pub async fn api() -> Result<(
    Vec<AdressClean>,
    Vec<MiljoeDataClean>,
    Vec<MiljoeDataClean>
), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    
    // Fetch all three datasets in parallel
    let (addresses, miljo, parkering) = tokio::join!(
        fetch_addresses(&client),
        fetch_miljoparkering(&client),
        fetch_parkeringsavgifter(&client)
    );
    
    Ok((addresses?, miljo?, parkering?))
}
```

### Pagination Handling

ArcGIS limits responses to 1000 features. AMP handles pagination automatically:

```rust
async fn fetch_all_features(client: &Client, base_url: &str) 
    -> Result<Vec<Feature>, Box<dyn std::error::Error>> 
{
    let mut all_features = Vec::new();
    let mut offset = 0;
    const PAGE_SIZE: usize = 1000;
    
    loop {
        let url = format!(
            "{}?resultOffset={}&resultRecordCount={}",
            base_url, offset, PAGE_SIZE
        );
        
        let response: FeatureCollection = client
            .get(&url)
            .send()
            .await?
            .json()
            .await?;
        
        let count = response.features.len();
        all_features.extend(response.features);
        
        if count < PAGE_SIZE {
            break;  // Last page
        }
        
        offset += PAGE_SIZE;
    }
    
    Ok(all_features)
}
```

### Data Transformation

**GeoJSON → Rust Structs:**

```rust
fn parse_address(feature: Feature) -> Option<AdressClean> {
    let coords = match feature.geometry {
        Geometry::Point { coordinates } => [
            Decimal::from_f64(coordinates[0])?,
            Decimal::from_f64(coordinates[1])?
        ],
        _ => return None,
    };
    
    Some(AdressClean {
        coordinates: coords,
        adress: feature.properties.get("ADRESS")?.as_str()?.to_string(),
        gata: feature.properties.get("GATA")?.as_str()?.to_string(),
        gatunummer: feature.properties.get("GATUNUMMER")?.as_str()?.to_string(),
        postnummer: feature.properties.get("POSTNUMMER")?.as_str()?.to_string(),
    })
}

fn parse_miljodata(feature: Feature) -> Option<MiljoeDataClean> {
    let coords = match feature.geometry {
        Geometry::LineString { coordinates } => [
            [
                Decimal::from_f64(coordinates[0][0])?,
                Decimal::from_f64(coordinates[0][1])?                ],
            [
                Decimal::from_f64(coordinates[1][0])?,
                Decimal::from_f64(coordinates[1][1])?
            ]
        ],
        _ => return None,
    };
    
    Some(MiljoeDataClean {
        coordinates: coords,
        info: feature.properties.get("INFO")?.as_str()?.to_string(),
        tid: feature.properties.get("TID")?.as_str()?.to_string(),
        dag: feature.properties.get("DAG")?.as_u64()? as u8,
    })
}
```

### Error Handling

**Strategy:** Graceful degradation

```rust
// Skip invalid features instead of failing entire dataset
let addresses: Vec<AdressClean> = features
    .into_iter()
    .filter_map(parse_address)  // Returns Option<AdressClean>
    .collect();
```

**Handled Errors:**
- Missing fields → Skip feature
- Invalid coordinates → Skip feature
- Network timeout → Retry with exponential backoff
- Invalid JSON → Return error

## Data Verification

**Module:** `core/src/checksum.rs`

### SHA256 Checksums

```rust
use sha2::{Sha256, Digest};

pub struct DataChecksum {
    pub miljoparkering: String,
    pub parkeringsavgifter: String,
    pub adresser: String,
    pub last_checked: String,
}

impl DataChecksum {
    pub async fn update_from_remote(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let client = reqwest::Client::new();
        
        // Fetch raw data
        let miljo_data = client.get(&self.miljoparkering_url).send().await?.bytes().await?;
        let parkering_data = client.get(&self.parkeringsavgifter_url).send().await?.bytes().await?;
        let adress_data = client.get(&self.adresser_url).send().await?.bytes().await?;
        
        // Compute checksums
        self.miljoparkering = format!("{:x}", Sha256::digest(&miljo_data));
        self.parkeringsavgifter = format!("{:x}", Sha256::digest(&parkering_data));
        self.adresser = format!("{:x}", Sha256::digest(&adress_data));
        self.last_checked = chrono::Utc::now().to_rfc3339();
        
        Ok(())
    }
    
    pub fn has_changed(&self, old: &DataChecksum) -> bool {
        self.miljoparkering != old.miljoparkering
            || self.parkeringsavgifter != old.parkeringsavgifter
            || self.adresser != old.adresser
    }
}
```

**Use Cases:**
- Detect when city updates data
- Trigger re-correlation on changes
- Monitor data freshness

## Performance Considerations

**Parallel Fetching:**
```rust
// Fetch all datasets simultaneously
let (addresses, miljo, parkering) = tokio::join!(
    fetch_addresses(&client),
    fetch_miljoparkering(&client),
    fetch_parkeringsavgifter(&client)
);
```

**Connection Pooling:**
```rust
// Reqwest automatically reuses connections
let client = reqwest::Client::builder()
    .pool_max_idle_per_host(10)
    .build()?;
```

**Timeouts:**
```rust
let client = reqwest::Client::builder()
    .timeout(Duration::from_secs(30))
    .build()?;
```

## Testing

**Mock API responses:**

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_parse_address() {
        let geojson = r#"
        {
            "type": "Feature",
            "geometry": {"type": "Point", "coordinates": [13.0, 55.6]},
            "properties": {
                "ADRESS": "Test 1",
                "GATA": "Test",
                "GATUNUMMER": "1",
                "POSTNUMMER": "211 22"
            }
        }
        "#;
        
        let feature: Feature = serde_json::from_str(geojson).unwrap();
        let addr = parse_address(feature).unwrap();
        
        assert_eq!(addr.adress, "Test 1");
    }
}
```

## Limitations

- **Rate Limiting:** None enforced by Malmö Open Data Portal (as of 2026)
- **Data Freshness:** Updated by city irregularly (weekly to monthly)
- **Pagination:** Max 1000 features per request (handled automatically)
- **Coordinate Precision:** 6 decimal places (~0.1 meter accuracy)

## Alternative Functions

**Fetch single dataset:**

```rust
// For testing or CLI with one dataset
pub async fn api_miljo_only() -> Result<(
    Vec<AdressClean>,
    Vec<MiljoeDataClean>
), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let (addresses, miljo) = tokio::join!(
        fetch_addresses(&client),
        fetch_miljoparkering(&client)
    );
    Ok((addresses?, miljo?))
}
```

## Related Documentation

- [Architecture](architecture.md) — System design
- [CLI Usage](cli-usage.md) — check-updates command
- [core/README.md](../core/README.md) — Core library guide
