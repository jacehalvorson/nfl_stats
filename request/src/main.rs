#[allow(dead_code, unused_variables, unused_macros)]

pub mod get_stats;
pub mod put_item;
pub mod types;

use std::env::args;
use std::time::Instant;
use crate::put_item::fetch_stats_and_add_to_table;
use crate::types::Category::*;
use crate::types::Category;

const ERROR_STRING: &str = "Usage: ./request.exe <start_year> <end_year> [passing|rushing|receiving|scrimmage|defense|kicking|punting|returns|scoring]";

const CATEGORIES: [Category; 9] = [
   Passing,
   Rushing,
   Receiving,
   Scrimmage,
   Defense,
   Kicking,
   Punting,
   Returns,
   Scoring
];

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
   let start = Instant::now();

   let args: Vec<String> = args().collect();
   if args.len() != 3 && args.len() != 4 {
      println!( "{}", ERROR_STRING );
      return Ok(());
   }

   let start_year = match args[1].parse::<i32>() {
      Ok( year ) => year,
      Err( _ ) => {
         println!( "{}", ERROR_STRING );
         return Ok(());
      }
   };

   let end_year = match args[2].parse::<i32>() {
      Ok( year ) => year,
      Err( _ ) => {
         println!( "{}", ERROR_STRING );
         return Ok(());
      }
   };

   if start_year >= end_year ||
      start_year < 1930 ||
      end_year > 2023
   {
      println!( "{}", ERROR_STRING );
      return Ok(());
   }

   // Check if a third argument was passed
   // If so, check if it's a valid category
   // If not, default to Passing
   let category = match args.get(3) {
      Some( category_string ) => {
         match category_string.as_str() {
            "passing" => Passing,
            "rushing" => Rushing,
            "receiving" => Receiving,
            "scrimmage" => Scrimmage,
            "defense" => Defense,
            "kicking" => Kicking,
            "punting" => Punting,
            "returns" => Returns,
            "scoring" => Scoring,
            _ => {
               println!( "{}", ERROR_STRING );
               return Ok(());
            }
         }
      }
      None => {
         // Default to starting with passing stats if not otherwise specified
         Passing
      }
   };

   for year in start_year..end_year {
      for cat in CATEGORIES.iter() {
         if year == start_year && *cat < category {
            continue;
         }
         fetch_stats_and_add_to_table( year, cat ).await?;
         println!( "{:?}: {} {:?} stats added to table", start.elapsed(), year, cat );
      }
   }

   Ok(())
}