// ... (keeping all the code above line 700 unchanged)
// Only showing the fix around line 700-720

                if let Some(restriction) = extract_restriction_from_info(&info) {
                    let gata_parts: Vec<&str> = result.address.split_whitespace().collect();
                    if gata_parts.len() >= 2 {
                        android_addresses.push(ParkingRestriction {
                            gata: gata_parts[0].to_string(),
                            gatunummer: gata_parts[1].to_string(),
                            postnummer: parse_postnummer(&result.postnummer),
                            adress: result.address.clone(),  // ADD THIS LINE
                            dag: restriction.dag,
                            tid: restriction.tid,
                            info,
                        });
                    }
                }
