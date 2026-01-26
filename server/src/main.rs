                const result = results[0];
                logMessage('PARSE', 'Result keys: ' + Object.keys(result).join(', '), 'info');
                logMessage('PARSE', 'Full result: ' + JSON.stringify(result), 'info');
                
                // API returns GeoJSON format with NAMN (name), TYPE, GEOM (geometry)
                const name = result.NAMN || result.adress || result.name || 'Unknown';
                
                // Parse GEOM - it's typically a string like "POINT(X Y)"
                let x, y;
                if (result.GEOM) {
                    const geomStr = result.GEOM;
                    logMessage('PARSE', 'GEOM format: ' + geomStr, 'info');
                    
                    // Try to extract coordinates from "POINT(X Y)" format
                    const pointMatch = geomStr.match(/POINT\s*\(([-\d.]+)\s+([-\d.]+)\)/);
                    if (pointMatch) {
                        x = parseFloat(pointMatch[1]);
                        y = parseFloat(pointMatch[2]);
                        logMessage('PARSE', 'Extracted from POINT: x=' + x + ', y=' + y, 'info');
                    }
                } else if (result.x && result.y) {
                    // Fallback to x, y properties if available
                    x = result.x;
                    y = result.y;
                    logMessage('PARSE', 'Using x, y properties: x=' + x + ', y=' + y, 'info');
                }
                
                logMessage('PARSE', 'Final coordinates: ' + name + ' (' + x + ', ' + y + ')', 'info');
                
                if (!x || !y || isNaN(x) || isNaN(y)) {
                    logMessage('ERROR', 'Invalid or missing coordinates. x=' + x + ', y=' + y, 'error');
                    updateStatus('❌ Coordinates not valid');
                    return;
                }
                
                logMessage('RESULT', 'Found: ' + name + ' at (' + x + ', ' + y + ')', 'success');
