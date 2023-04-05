#[allow(dead_code, unused_variables, unused_macros)]

use std::path::Path;
use std::fs::File;
use std::io::prelude::*;
use std::error::Error;
use reqwest::{get, Response};
use scraper::{Html, Selector};

#[derive(Debug)]
struct StatsError;

macro_rules! debug_print {
   ( $printed_object: ident ) => {
       println!( "DEBUG: {}:{}:{}\n{}: {:?}\n", file!(), line!(), column!(), stringify!($printed_object), $printed_object );
   };
}

pub async fn get_stats_and_write_to_html( url: &str ) -> Result<(), Box<dyn Error>> {
   // Send GET request to the URL and get HTML in plaintext
   let response: Response = get(url).await?;
   let text: String = response.text().await?;

   // Select the table with NFL stats
   let document = Html::parse_document( &text );
   let _head_selector = Selector::parse(".stats_table > thead > tr > th")?;
   let _body_selector = Selector::parse(".stats_table > tbody > tr")?;
   let table_container_selector = Selector::parse(".table_container")?;

   // Grab innerHTML of the headers for the table
   let table_container_html = document.select( &table_container_selector ).next().unwrap();
   let table_container_html_string = table_container_html.inner_html();

   // Write Stats struct to JSON file
   let path = Path::new( r"data/stats_table.html" );
   let mut file: File = File::create( path ).unwrap();
   file.write_all( table_container_html_string.as_bytes() ).unwrap();

   return Ok( () );
}