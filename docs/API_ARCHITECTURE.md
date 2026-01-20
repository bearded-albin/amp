# API Architecture Documentation

## Overview

The ArcGIS API client (`api.rs`) provides a robust interface for fetching and transforming geospatial data from Malmö's ArcGIS Feature Services. This document covers the architecture, data flow, and implementation details.

**Reference:** [REF-API-001]

## Core Components

### ArcGISClient Structure

The main client struct wraps a `reqwest::Client` for HTTP operations.

**Key Features:**
- Automatic pagination handling
- Geospatial data transformation (GeoJSON)
- Error resilience and recovery

**Reference:** [REF-API-002]

## Feature Fetching Pipeline

### `fetch_all_features()`

Fetches all features from an ArcGIS Feature Service with automatic pagination.

**Parameters:**
- `service_url`: Base URL of the ArcGIS FeatureServer
- `layer_id`: Layer identifier within the service

**Pagination Strategy:**
- Uses `resultOffset` and `resultRecordCount` parameters
- Batch size: 1000 records per request
- Continues until `exceeded_transfer_limit` is false or fewer records than batch size are returned

**Reference:** [REF-API-003]

## Data Transformation

### GeoJSON Point Extraction

`extract_point_from_geojson()` converts ArcGIS geometry objects to coordinate pairs.

**Precision Handling:**
- Uses `Decimal::from_f64_retain()` to preserve coordinate precision
- Maintains at least 7 decimal places (±0.111 meters precision at equator)
- Prevents floating-point precision loss in distance calculations

**Reference:** [REF-API-004]

### GeoJSON Polygon Bounding Box Extraction

`extract_polygon_from_geojson()` extracts first and last coordinates from polygon rings to create bounding box representations.

**Used for:**
- Environmental parking zones
- Area-based restrictions

**Reference:** [REF-API-005]

## Data Structure Conversions

### Address Conversion: `to_adress_clean()`

Transforms raw ArcGIS features into `AdressClean` structures.

**Field Mapping:**
| ArcGIS Field | Fallback Fields | Target Field | Type |
|---|---|---|---|
| PostalCode | postalcode, POSTALCODE | postnummer | u16 |
| FullAddress | Address, FULLADDRESS | adress | String |
| StreetName | Street, STREETNAME | gata | String |
| StreetNumber | Number, STREETNUMBER | gatunummer | String |
| geometry | - | coordinates | [Decimal; 2] |

**Filtering:** Entries with missing fields are skipped (Option handling).

**Reference:** [REF-API-006]

### Parking Zone Conversion: `to_miljoe_clean()`

Transforms raw ArcGIS features into `MiljoeDataClean` structures.

**Field Mapping:**
| ArcGIS Field | Fallback Fields | Target Field | Type |
|---|---|---|---|
| Name | Info, NAME | info | String |
| Time | Tid, TIME | tid | String |
| Day | Dag, DAY | dag | u8 |
| geometry | - | coordinates | [[Decimal; 2]; 2] |

**Filtering:** Entries with missing fields are skipped (Option handling).

**Reference:** [REF-API-007]

## Data Sources

### Malmö Addresses

**Service URL:** `https://services3.arcgis.com/GVgbJbqm8hXASVYi/ArcGIS/rest/services/Malmo_Sweden_Addresses/FeatureServer`

**Layer:** 0

**Provides:**
- Complete address database for Malmö
- Precise coordinates (7+ decimal places)
- Postal code information

**Reference:** [REF-API-008]

### Environmental Parking Zones

**Service URL:** `https://gis.malmo.se/arcgis/rest/services/FGK_Parkster_Map/FeatureServer`

**Layer:** 1

**Provides:**
- Parking zone boundaries (polygon coordinates)
- Time-based restrictions (tid)
- Day-of-week information (dag)
- Zone information and naming

**Reference:** [REF-API-009]

## Error Handling

### Resilience Patterns

1. **Network Failures:** Propagated via `Result<T, Box<dyn std::error::Error>>`
2. **Parsing Failures:** Skipped gracefully (Option/filter_map pattern)
3. **Missing Fields:** Records excluded from output (filter_map with Option)

**Reference:** [REF-API-010]

## Performance Considerations

### Pagination
- Reduces memory footprint for large datasets
- Prevents timeout on slow connections
- Allows parallel processing of batches

**Reference:** [REF-API-011]

### Async/Await
- Non-blocking I/O operations
- Enables concurrent API calls
- Uses `#[tokio::main]` runtime

**Reference:** [REF-API-012]

## Integration

The `api()` function serves as the main entry point, returning a tuple of:
- `Vec<AdressClean>`: All Malmö addresses with coordinates
- `Vec<MiljoeDataClean>`: All parking zones with restrictions

These vectors are ready for correlation analysis without file I/O.

**Reference:** [REF-API-013]

## Field Mapping Flexibility

The triple-fallback pattern (`field1 OR field2 OR field3`) ensures compatibility with:
- Multiple API versions
- Different naming conventions (UPPERCASE, lowercase, MixedCase)
- External data sources

**Reference:** [REF-API-014]
