#[allow(dead_code, unused_variables, unused_macros)]

use std::error::Error;
use reqwest::Client;
use scraper::{Html, Selector};
use crate::types::Category::*;
use crate::types::*;

// Constants
const TEAM_COLUMN: usize = 3;
const QB_REC_COLUMN: usize = 7;
const YPC_COLUMN: usize = 20;

pub async fn get_stats( year: i32, category: &str ) -> Result<DynamoDBItem, Box<dyn Error>> {
   // Construct the URL
   let url = format!( "https://www.pro-football-reference.com/years/{year}/{category}.htm", year=year, category=category );

   // Send GET request to the URL and get HTML in plaintext
   let client = Client::new();
   let response = client.get(url).send().await?;
   let mut text = String::new();

   if response.status().is_success() {
      text.push_str( &response.text().await? );
   } else {
      return Err( Box::new( std::io::Error::new(
               std::io::ErrorKind::Other, format!( "GET request failed for {} {} stats", year, category ) ) ) );
   }

   // Parse the HTML to get stats table represented as strings
   let document = Html::parse_document( &text );
   
   // Selectors for the table headers and table rows
   let head_selector = Selector::parse(".stats_table > thead > tr:not(.over_header) > th")?;
   let body_selector = Selector::parse(".stats_table > tbody > tr:not(.thead)")?;
   let a_selector = Selector::parse("a")?;
   let td_selector = Selector::parse("td")?;

   // Grab innerHTML of the headers for the table
   let attribute_list = document.select( &head_selector ).map( |element| {
         element.inner_html()
      })
      .collect::<Vec<String>>();

   // Create a vector of players, each of which is a vector of their stats
   let mut stats_matrix: Vec<Vec<String>> = Vec::new( );
   // Grab a list of <tr> tags
   let rows = document.select( &body_selector );

   // Iterate over the table rows and collect data from each
   for ( row_index, tr ) in rows.enumerate()
   {
      // Create a vector of stats for this player
      let mut player_vector: Vec<String> = Vec::new( );
      // Player rank 1-n
      player_vector.push( ( row_index + 1 ).to_string() );

      for ( col_index, td ) in tr.select( &td_selector ).enumerate( )
      {
         // Each <td> may contain "<a>Stat</a>"" or just "Stat"
         match td.select( &a_selector ).next() {
            Some( a ) => {
               // Grab the innerHTML within the <a> tag
               player_vector.push( filter_stat( a.inner_html(), col_index, Passing ) );
            }
            None => { 
               // There are no <a> tags, so just grab the innerHTML of the td
               player_vector.push( filter_stat( td.inner_html(), col_index, Passing ) );
            }
         }
      }

      // Add the player's data vector to the data matrix
      stats_matrix.push( player_vector );
   }

   let json_object = DynamoDBItem {
      id: format!( "{year}{category}", year=year, category=category ),
      attributes: attribute_list,
      players: stats_matrix
   };

   return Ok( json_object );
}

// Filter the stats based on the column index.
pub fn filter_stat( stat: String, col_index: usize, position: Category ) -> String {
   match col_index {
      // For the team column, use more common names
      TEAM_COLUMN => {
         match stat.as_str() {
            "NWE" => "NE".to_string(),
            "GNB" => "GB".to_string(),
            "KAN" => "KC".to_string(),
            "SFO" => "SF".to_string(),
            "NOS" => "NO".to_string(),
            "TAM" => "TB".to_string(),
            "NOR" => "NO".to_string(),
            _ => stat
         }
      },

      // For the QBRec column, display 0-0-0 for empty stats
      QB_REC_COLUMN => {
         match stat.as_str() {
            "" => "0-0-0".to_string(),
            _ => stat
         }
      },

      // For the YPC column, display -- for empty stats (players with 0 carries)
      YPC_COLUMN => {
         match stat.as_str() {
            "" => "--".to_string(),
            _ => stat
         }
      },

      // For all other columns, display 0 for empty stats
      _ => {
         match stat.as_str() {
            "" => "0".to_string(),
            _ => stat
         }
      }
   }
}