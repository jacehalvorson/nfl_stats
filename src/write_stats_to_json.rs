use std::path::Path;
use std::fs::File;
use std::io::prelude::*;
use reqwest::{get, Response};
use scraper::{Html, Selector};
use serde::{Serialize};
use serde_json::{Value, Result};

#[derive(Debug, Serialize)]
struct Stats {
   attributes: Vec<String>,
   data: Vec<QBPlayer>
}

#[derive(Debug, Serialize)]
struct QBPlayer {
   name: String,
   team: String,
   age: u8,
   position: String,
   games: u8,
   games_started: u8,
   qb_record: String,
   completions: u16,
   attempts: u16,
   completion_percentage: f32,
   yards: u32,
   touchdowns: u8,
   touchdown_percentage: f32,
   interceptions: u8,
   interception_percentage: f32,
   first_downs: u16,
   longest: u16,
   yards_per_attempt: f32,
   adjusted_yards_per_attempt: f32,
   yards_per_completion: f32,
   yards_per_game: f32,
   passer_rating: f32,
   qb_rating: f32,
   sacks: u8,
   sack_yards: u16,
   sack_percentage: f32,
   net_yards_per_attempt: f32,
   adjusted_net_yards_per_attempt: f32,
   fourth_quarter_comebacks: u8,
   game_winning_drives: u8
}

macro_rules! debug_print {
    ( $printed_object: ident ) => {
        println!( "DEBUG: {}:{}:{}\n{}: {:?}\n", file!(), line!(), column!(), stringify!($printed_object), $printed_object );
    };
}

fn write_to_json( _path: &Path, data: &Stats ) -> Result<()> {
   let json_string = serde_json::to_string(data)?;
   debug_print!( json_string );
   debug_print!( data );

   // Create a JSON file and write to it
   let mut file: File = File::create( "data/stats.json" ).unwrap();
   file.write_all( json_string.as_bytes() ).unwrap();
   return Ok( () );
}

pub async fn write_stats_to_json( url: &str ) -> Result<()> {
   // Send GET request to the URL and get HTML in plaintext
   let response: Response = get(url).await.unwrap();
   let text: String = response.text().await.unwrap();

   // Select the table with NFL stats
   let document = Html::parse_document( &text );
   let selector = Selector::parse(".stats_table > thead > tr > th").unwrap();

   // Grab headers for the table
   let headers: Vec<String> = document.select( &selector )
                              .map( |element| 
                                    element.inner_html() )
                              .collect::<Vec<String>>();
   debug_print!( headers );                                  

   let fake_qbs = vec![ QBPlayer {
      name: "foo".to_string(),
      team: "bar".to_string(),
      age: 1,
      position: "baz".to_string(),
      games: 2,
      games_started: 3,
      qb_record: "qux".to_string(),
      completions: 4,
      attempts: 5,
      completion_percentage: 6.0,
      yards: 7,
      touchdowns: 8,
      touchdown_percentage: 9.0,
      interceptions: 10,
      interception_percentage: 11.0,
      first_downs: 12,
      longest: 13,
      yards_per_attempt: 14.0,
      adjusted_yards_per_attempt: 15.0,
      yards_per_completion: 16.0,
      yards_per_game: 17.0,
      passer_rating: 18.0,
      qb_rating: 19.0,
      sacks: 20,
      sack_yards: 21,
      sack_percentage: 22.0,
      net_yards_per_attempt: 23.0,
      adjusted_net_yards_per_attempt: 24.0,
      fourth_quarter_comebacks: 25,
      game_winning_drives: 26
   } ];

   // Fill Stats struct
   let json_data: Stats = Stats { attributes: headers, data: fake_qbs };

   // Write Stats struct to JSON file
   let path = Path::new( r"data/stats.json" );
   write_to_json( &path, &json_data ).unwrap();

   return Ok( () );
}