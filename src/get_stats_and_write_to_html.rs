#[allow(dead_code, unused_variables, unused_macros)]

use std::path::Path;
use std::fs::File;
use std::io::prelude::*;
use std::error::Error;
use reqwest::{get, Response};
use scraper::{Html, Selector};

pub async fn get_stats_and_write_to_html( url: &str ) -> Result<(), Box<dyn Error>> {
   // Send GET request to the URL and get HTML in plaintext
   let response: Response = get(url).await?;
   let text: String = response.text().await?;

   // Select the table with NFL stats
   let document = Html::parse_document( &text );
   let table_container_selector = Selector::parse(".table_container")?;

   // Grab innerHTML of the headers for the table
   let table_container_html = document.select( &table_container_selector ).next().unwrap();
   let table_container_html_string = table_container_html.inner_html();

   // Write the entirety of the table container to an HTML file
   let path = Path::new( r"data/stats_table.html" );
   let mut file: File = File::create( path ).unwrap();
   file.write_all( table_container_html_string.as_bytes() ).unwrap();

   return Ok( () );
}