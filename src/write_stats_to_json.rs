use std::path::Path;
use std::fs::File;
use std::io::prelude::*;
use reqwest::{get, Response};
use scraper::{Html, Selector};
use serde::{Serialize};
use serde_json::{Result};

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

#[allow(unused_macros)]
macro_rules! debug_print {
    ( $printed_object: ident ) => {
        println!( "DEBUG: {}:{}:{}\n{}: {:?}\n", file!(), line!(), column!(), stringify!($printed_object), $printed_object );
    };
}

#[allow(dead_code)]
fn write_to_json( _path: &Path, data: &Stats ) -> Result<()> {
   // Serialize the data structure int JSON
   let json_string = serde_json::to_string(data)?;
   
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
   let _head_selector = Selector::parse(".stats_table > thead > tr > th").unwrap();
   let _body_selector = Selector::parse(".stats_table > tbody > tr").unwrap();
   let table_container_selector = Selector::parse(".table_container").unwrap();

   // Grab innerHTML of the headers for the table
   let table_container_html: String = document.select( &table_container_selector ).next().unwrap().inner_html();

   // Write Stats struct to JSON file
   let path = Path::new( r"data/stats_table.html" );
   let mut file: File = File::create( path ).unwrap();
   file.write_all( table_container_html.as_bytes() ).unwrap();

   return Ok( () );
}