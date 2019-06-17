# holochain-geospatial
Geospatial library for holochain

## Overview
The purpose of the library is to enable storage and retreival of geospatial points to a holochain DHT. The following geospatial querying functionality is planned:

* near - find points within a specified distance of point
* within - find points within a specified rectangle

## Design

![Globe](http://s2geometry.io/devguide/img/s2curve-tiny.gif)

The image below shows a covering of the united states with S2 cells at level 6 (image generated with https://s2.sidewalklabs.com/regioncoverer). At this level 24k cells are required to cover the entire globe.

![S2 Cell Covering](https://imgur.com/upit5Rl.jpg)
