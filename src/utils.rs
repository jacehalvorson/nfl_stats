use std::path::Path;
use std::fs::File;
use std::io::BufWriter;
use reqwest::{get, Response, Error};
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use serde_json::{Result, to_writer};

struct Stats {
   headers: Vec<String>,
   data: Vec<Vec<String>>
}

struct PassingHeaders {
   rank: String,
   player: String,
   team: String,
   age: String,
   position: String,
   games: String,
   games_started: String,
   qb_record: String,
   completions: String,
   attempts: String,
   completion_percentage: String,
   yards: String,
   touchdowns: String,
   touchdown_percentage: String,
   interceptions: String,
   interception_percentage: String,
   first_downs: String,
   longest: String,
   yards_per_attempt: String,
   adjusted_yards_per_attempt: String,
   yards_per_completion: String,
   yards_per_game: String,
   passer_rating: String,
   qb_rating: String,
   sacks: String,
   sack_yards: String,
   sack_percentage: String,
   net_yards_per_attempt: String,
   adjusted_net_yards_per_attempt: String,
   fourth_quarter_comebacks: String,
   game_winning_drives: String
}

fn write_to_json( _path: &Path, data: &Vec<String> ) -> Result<()> {
   let mut json_string: String = String::from("{[\"products\":");
   json_string.push_str( &data.join(",") );
   json_string.push_str( "]}");
   println!( "{}", json_string );

   // Create a JSON file and write to it
   let file: File = File::create( "stats.json" ).unwrap();
   let writer: BufWriter<File> = std::io::BufWriter::new( file );
   serde_json::to_writer(writer, &json_string).unwrap();
   return Ok( () );
}

pub async fn get_page() -> Result<Vec<String>> {
   // Send GET request to the URL and get HTML in plaintext
   let url: &str = "https://www.pro-football-reference.com/years/2022/passing.htm";
   let response: Response = get(url).await.unwrap();
   let text: String = response.text().await.unwrap();

   // Select the table with NFL stats
   let document = Html::parse_document( &text );
   let selector = Selector::parse(".stats_table > thead > tr > th").unwrap();

   // Grab headers for the table
   let mut headers: Vec<String> = Vec::new();
   for element in document.select( &selector ) {
      headers.push( element.inner_html() );
   }

   // Write headers to JSON file
   let path = Path::new( r"..\data\stats.json" );
   write_to_json( &path, &headers ).unwrap();

   return Ok( headers );
}