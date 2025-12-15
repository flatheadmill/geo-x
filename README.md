Index [GeoFabrik](https://www.geofabrik.de/) data in Tantivy using the spatial field type.

Choose any of the data sets from the [GeoFabrik
Downloads](https://download.geofabrik.de/). Download the `.osm.pbf` format.

To convert to GeoJSON you will need to install the [Osmium
Tool](https://osmcode.org/osmium-tool/). It installs from Homebrew on OS X and
there are packages availble for Fedora and Debian/Ubuntu.

To extract a file.

```
osmium export us-251112.osm.pbf -f geojsonseq  | tr -d $'\x1e' > us.jsonl
```

Indexing is performed using index. The first argument is a JSONL file with
GeoJSON Feature types. The second argument is a directory.

```
cargo run --bin index -- ./us.jsonl ./index
```
