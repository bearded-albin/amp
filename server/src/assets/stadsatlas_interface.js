const logs = [];
const addressToSearch = document.querySelector('.header .address')?.textContent?.trim() || 'Unknown';

function logMessage(category, message, type = 'info') {
    const timestamp = new Date().toLocaleTimeString();
    const logEntry = {timestamp, category, message, type};
    logs.push(logEntry);

    console.log('[AMP] [' + timestamp + '] [' + category + '] ' + message);

    const logsDiv = document.getElementById('message-logs');
    if (logsDiv) {
        const entry = document.createElement('div');
        entry.className = 'log-entry ' + type;
        entry.innerHTML = '<span class="log-timestamp">[' + timestamp + ']</span> <strong>' + category + ':</strong> ' + message;
        logsDiv.appendChild(entry);
        logsDiv.scrollTop = logsDiv.scrollHeight;
    }
}

function updateStatus(status, statusId = 'search-status') {
    const statusDiv = document.getElementById(statusId);
    if (statusDiv) {
        statusDiv.textContent = status;
    }
    // Also update the main status indicator
    const indicator = document.getElementById('status-indicator');
    if (indicator) {
        indicator.textContent = status;
    }
}

function switchTab(event, tabNumber) {
    const tabs = document.querySelectorAll('.tab-content');
    tabs.forEach(function(tab) { tab.classList.remove('active'); });
    const btns = document.querySelectorAll('.tab-btn');
    btns.forEach(function(btn) { btn.classList.remove('active'); });
    document.getElementById('tab' + tabNumber).classList.add('active');
    event.target.classList.add('active');
}

async function searchAddress() {
    logMessage('SEARCH', 'Starting address search for: ' + addressToSearch, 'info');
    updateStatus('⏳ Searching for: ' + addressToSearch);

    try {
        const searchUrl = 'https://geo.malmo.se/api/search?q=' + encodeURIComponent(addressToSearch);
        logMessage('API', 'Calling: ' + searchUrl.substring(0, 60) + '...', 'info');

        const response = await fetch(searchUrl);
        if (!response.ok) {
            throw new Error('API returned status ' + response.status);
        }

        const results = await response.json();
        logMessage('API', 'Response received with ' + results.length + ' results', 'success');

        if (results.length === 0) {
            logMessage('RESULT', 'No address found matching: ' + addressToSearch, 'warning');
            updateStatus('❌ Address not found in Malmö');
            return;
        }

        const result = results[0];
        logMessage('PARSE', 'Result keys: ' + Object.keys(result).join(', '), 'info');
        
        // Parse Malmö API response with WKT GEOM format
        const name = result.NAMN || result.name || result.adress || 'Unknown';
        let x, y;
        
        // Extract from WKT POINT format: POINT(X Y)
        if (result.GEOM) {
            const match = result.GEOM.match(/POINT\s*\(([^\s]+)\s+([^)]+)\)/);
            if (match) {
                x = parseFloat(match[1]);
                y = parseFloat(match[2]);
                logMessage('PARSE', 'Extracted WKT: x=' + x + ', y=' + y, 'info');
            }
        }
        
        // Fallback to x, y properties
        if (!x || !y) {
            x = result.x;
            y = result.y;
            if (x && y) logMessage('PARSE', 'Using x, y properties: x=' + x + ', y=' + y, 'info');
        }
        
        if (!x || !y || isNaN(x) || isNaN(y)) {
            logMessage('ERROR', 'Missing coordinates in response', 'error');
            updateStatus('❌ Coordinates not found');
            return;
        }
        
        logMessage('RESULT', 'Found: ' + name + ' at (' + x + ', ' + y + ')', 'success');

        // Build StadsAtlas URL with multiple layer parameter formats
        // Try different parameter combinations as StadsAtlas may use different conventions
        
        // Format 1: Standard hash parameters with layer ID
        const mapUrl = 'https://stadsatlas.malmo.se/stadsatlas/#center=' + x + ',' + y + 
                       '&zoom=18' +
                       '&pin=' + x + ',' + y +
                       '&layers=miljoparkering_l' +
                       '&layerIds=miljoparkering_l' +
                       '&visibleLayers=miljoparkering_l';
        
        logMessage('MAP', 'Building StadsAtlas URL with miljöparkering layer...', 'info');
        logMessage('MAP', 'Trying multiple layer parameter formats', 'info');
        logMessage('MAP', 'URL: ' + mapUrl.substring(0, 100) + '...', 'info');

        // Load in iframe
        const mapContainer = document.getElementById('map-container');
        const iframe = document.getElementById('stadsatlas-iframe');
        const placeholder = document.querySelector('.map-placeholder');
        
        if (mapContainer && iframe) {
            iframe.src = mapUrl;
            iframe.style.display = 'block';
            if (placeholder) {
                placeholder.style.display = 'none';
            }
            
            logMessage('MAP', 'Map loaded in persistent container at top', 'success');
            logMessage('LAYER', 'Attempted to activate miljöparkering layer via URL parameters', 'info');
            logMessage('LAYER', 'If layer not visible, manually activate it using the Layers panel in the map', 'warning');
            updateStatus('✅ Map loaded: ' + name);
            
            // After iframe loads, try to send a message to activate the layer
            iframe.onload = function() {
                logMessage('MAP', 'Iframe loaded successfully', 'success');
                logMessage('LAYER', 'Note: Layer activation via URL may require manual confirmation in StadsAtlas UI', 'info');
            };
        } else {
            logMessage('ERROR', 'Map container not found in DOM', 'error');
            updateStatus('❌ Error: Map container not available');
        }

    } catch (error) {
        logMessage('ERROR', 'Search failed: ' + error.message, 'error');
        updateStatus('❌ Error: ' + error.message);
    }
}

// Initial status
window.addEventListener('load', function() {
    logMessage('READY', 'AMP Testing Interface loaded. Map section is persistent at top, tabs cycle below.', 'info');
});