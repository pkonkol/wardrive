Scan as much as possible of available wireless protocols using RPI0 (or similar)
nodes (in Rust) and send them to the master aggregator & DB (in Go).  

# master
- go
- postgreSQL
- API server for receiving node updates
-            endpoint with system state (HTTP)

# node
- Rust
- rpi0
- API client communicates with the master, sending updates 
-     auth before update
- API server provides basic status
- scans WiFi, BT, GPS, GSM? (and possibly more)
- code structured into modules for each
- each module working concurrently
- more detailed scans for interesting targets
- local persistent result caching (b4 received by master)
- binary that can start with system & continue where left with power supply drop