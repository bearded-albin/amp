# Iframe Injection Script

This script handles auto-reloading the page and injecting address searches into the StadsAtlas iframe.

## Key Features

1. **Auto-Reload**: First page load triggers a reload after 1.5 seconds to allow iframe initialization
2. **Address Injection**: After reload, script waits for iframe and injects the address into the search field
3. **Cross-Origin Handling**: Attempts to catch cross-origin restrictions gracefully
4. **Multiple Selectors**: Tries different selectors to find the search input
5. **Event Simulation**: Fires input, change, and keydown (Enter) events to trigger search

## Replace in main.rs

Replace the `<script>` section in `create_tabbed_interface_page()` function with the code that includes:

- Page reload detection on load event
- Iframe search input detection
- Address injection with proper event firing
- Error handling and logging

## Status

Needs to be manually merged into main.rs due to format parameter size restrictions.