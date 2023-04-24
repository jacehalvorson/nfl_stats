#[allow(dead_code, unused_variables, unused_macros)]

use std::error::Error;
use reqwest::Client;
use scraper::{Html, Selector};
use crate::types::Category::*;
use crate::types::*;

pub async fn get_stats( year: i32, category: &Category ) -> Result<DynamoDBItem, Box<dyn Error>> {
   // Construct the URL
   let category_string: &str = match category {
      Passing => "passing",
      Rushing => "rushing",
      Receiving => "receiving",
      Scrimmage => "scrimmage",
      Defense => "defense",
      Kicking => "kicking",
      Punting => "punting",
      Returns => "returns",
      Scoring => "scoring"
   };
   let url = format!( "https://www.pro-football-reference.com/years/{}/{}.htm", year, category_string );

   // Send GET request to the URL and get HTML in plaintext
   let client = Client::new();
   let response = client.get(url).send().await?;
   let mut text = String::new();

   if response.status().is_success() {
      text.push_str( &response.text().await? );
   } else {
      return Err( Box::new( std::io::Error::new(
               std::io::ErrorKind::Other, format!( "GET request failed for {} {} stats", year, category_string ) ) ) );
   }

   // Parse the HTML to get stats table represented as strings
   let document = Html::parse_document( &text );
   
   // Selectors for the table headers and table rows
   let head_selector = Selector::parse(".stats_table > thead > tr:not(.over_header) > th")?;
   let body_selector = Selector::parse(".stats_table > tbody > tr:not(.thead)")?;
   let a_selector = Selector::parse("a")?;
   let td_selector = Selector::parse("td")?;

   // Grab innerHTML of the headers for the table
   let attribute_list: Vec<String> = document.select( &head_selector ).map( |element| {
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
               player_vector.push( filter_stat( a.inner_html(), attribute_list[ col_index+1 ].clone() ) );
            }
            None => { 
               // There are no <a> tags, so just grab the innerHTML of the td
               player_vector.push( filter_stat( td.inner_html(), attribute_list[ col_index+1 ].clone() ) );
            }
         }
      }

      // Add the player's data vector to the data matrix
      stats_matrix.push( player_vector );
   }

   let json_object = DynamoDBItem {
      // The id is the year and category concatenated (ex. 2017receiving)
      id: format!( "{}{}", year, category_string ),
      attributes: attribute_list,
      players: stats_matrix
   };

   return Ok( json_object );
}

// Filter the stats based on the column index.
pub fn filter_stat( stat: String, attribute: String ) -> String {
   match attribute.as_str() {
      // For the team column, use more common names
      "Tm" => {
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
      "QBrec" => {
         match stat.as_str() {
            "" => "0-0-0".to_string(),
            _ => stat
         }
      },

      // For the columns dependent on a denominator, display -- for empty denominators.
      // This describes players with 0 carries, targets, touches, or passing attempts
      "QBR" | "Y/C" | "Y/R" | "Y/Tgt" | "Y/Tch" | "Y/A" | "Y/Rt" | "Lng" | "Ctch%" | "FG%" | "XP%" | "TB%" | "KOAvg" => {
         match stat.as_str() {
            "" => "--".to_string(),
            _ => stat
         }
      },

      // For the floating-point values guaranteed to have a denominator, display 0.0 for empty stats
      "R/G" | "Y/G" | "A/G" | "Y/Tch" => {
         match stat.as_str() {
            "" => "0.0".to_string(),
            _ => stat
         }
      },

      // For columns we don't want to display 0 in (exceptions to default rule)
      "Pos" => {
         stat
      }

      // For all other columns, display 0 for empty stats
      _ => {
         match stat.as_str() {
            "" => "0".to_string(),
            _ => stat
         }
      }
   }
}