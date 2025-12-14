use std::fs::File;
use std::io::{BufRead, BufReader};
use tantivy::collector::TopDocs;
use tantivy::query::SpatialQuery;
use tantivy::schema::{SPATIAL, STORED, Schema, TEXT, Value};
use tantivy::spatial::point::GeoPoint;
use tantivy::{Index, IndexWriter, TantivyDocument};
fn main() -> tantivy::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let input = &args[1];
    let output= &args[2];
    let mut schema_builder = Schema::builder();
    schema_builder.add_json_field("properties", STORED | TEXT);
    schema_builder.add_spatial_field("geometry", STORED | SPATIAL);
    schema_builder.add_text_field("type", STORED);

    let schema = schema_builder.build();
    let index = Index::create_in_dir(output, schema.clone())?;
    let mut index_writer: IndexWriter = index.writer(50_000_000)?;
    let file = File::open(input)?;
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line?;
        let doc = TantivyDocument::parse_json(&schema, &line)?;
        index_writer.add_document(doc)?;
    }
    index_writer.commit()?;

    let reader = index.reader()?;
    let searcher = reader.searcher();
    let field = schema.get_field("geometry").unwrap();
    let query = SpatialQuery::new(
        field,
        [
            GeoPoint {
                lon: -99.49,
                lat: 45.56,
            },
            GeoPoint {
                lon: -99.45,
                lat: 45.59,
            },
        ],
        tantivy::query::SpatialQueryType::Intersects,
    );
    let hits = searcher.search(&query, &TopDocs::with_limit(10))?;
    for (_score, doc_address) in &hits {
        let retrieved_doc: TantivyDocument = searcher.doc(*doc_address)?;
        if let Some(field_value) = retrieved_doc.get_first(field)
            && let Some(geometry_box) = field_value.as_value().into_geometry()
        {
            println!("Retrieved geometry: {:?}", geometry_box);
        }
    }
    Ok(())
}
