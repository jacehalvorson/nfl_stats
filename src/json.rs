#[allow(dead_code, unused_variables, unused_macros)]

use std::fs::File;
use std::io::prelude::*;
use std::error::Error;
use reqwest::{get, Response};
use scraper::{Html, Selector};

macro_rules! debug_print {
   ( $printed_object: ident ) => {
       println!( "DEBUG: {}:{}:{}\n{}: {:?}\n", file!(), line!(), column!(), stringify!($printed_object), $printed_object );
   };
}

pub async fn get_stats_and_write_to_json( url: &str ) -> Result<(), Box<dyn Error>> {
   // Send GET request to the URL and get HTML in plaintext
   let response: Response = get(url).await?;
   let text: String = response.text().await?;

   let head_selector = Selector::parse(".stats_table > thead > tr:not(.over_header) > th")?;
   let body_selector = Selector::parse(".stats_table > tbody > tr")?;
   let a_selector = Selector::parse("a")?;
   let td_selector = Selector::parse("td")?;

   let document = Html::parse_document( &text );
   
   // Grab innerHTML of the headers for the table
   let header_list = document.select( &head_selector ).map( |element| {
         element.inner_html()
      })
      .collect::<Vec<String>>();

   // Create a vector of players, each of which is a vector of their stats
   let mut data_matrix: Vec<Vec<String>> = Vec::new( );
   // Grab a list of <tr> tags
   let rows = document.select( &body_selector );

   // Iterate over the table rows and collect data from each
   for ( index, tr ) in rows.enumerate()
   {
      // Create a vector of stats for this player
      let mut player_vector: Vec<String> = Vec::new( );
      // Player rank 1-n
      player_vector.push( ( index + 1 ).to_string() );

      for td in tr.select( &td_selector )
      {
         // Each <td> may contain "<a>Stat</a>"" or just "Stat"
         match td.select( &a_selector ).next() {
            Some( a ) => {
               // Grab the innerHTML within the <a> tag
               player_vector.push( a.inner_html() );
            }
            None => { 
               // There are no <a> tags, so just grab the innerHTML of the td
               player_vector.push( td.inner_html() );
            }
         }
      }

      // Add the player's data vector to the data matrix
      data_matrix.push( player_vector );
   }

   let json_string = format!(
r#"{{
   "headers": {header_list:?},
   "rows": {data_matrix:?}
}}"#,
      header_list = header_list,
      data_matrix = data_matrix );

   // Write this formatted string to a JSON file
   let mut file: File = File::create( "stats_table.json" ).unwrap();
   file.write_all( json_string.as_bytes() ).unwrap();

   return Ok( () );
}