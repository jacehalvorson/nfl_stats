#[allow(dead_code, unused_variables, unused_macros)]

use serde::Serialize;
use serde_json::Result;

#[derive(Serialize)]
struct Stats {
   headers: Vec<String>,
   rows: Vec<Vec<String>>,
}

fn write_to_json( _path: &Path, data: &Stats ) -> Result<()> {
   // Serialize the data structure int JSON
   let json_string = serde_json::to_string(data)?;
   
   // Create a JSON file and write to it
   let mut file: File = File::create( "data/stats.json" ).unwrap();
   file.write_all( json_string.as_bytes() ).unwrap();
   return Ok( () );
}