#[allow(dead_code, unused_variables, unused_macros)]

pub mod get_stats;
pub mod put_item;
pub mod types;

use std::env::args;
use crate::put_item::fetch_stats_and_add_to_table;
use crate::get_stats::get_stats;
use crate::types::Category::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
   let args: Vec<String> = args().collect();
   if args.len() != 3 {
      println!( "Usage: cargo run --release <year> <passing|rushing|receiving|scrimmage|defense|kicking|punting|returns|scoring>" );
      return Ok(());
   }

   let year = match args[1].parse::<i32>() {
      Ok( year ) => year,
      Err( _ ) => {
         println!( "Usage: cargo run --release <year> <passing|rushing|receiving|scrimmage|defense|kicking|punting|returns|scoring>" );
         return Ok(());
      }
   };

   let category = match args[2].as_str() {
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
         println!( "Usage: cargo run --release <year> <passing|rushing|receiving|scrimmage|defense|kicking|punting|returns|scoring>" );
         return Ok(());
      }
   };

   for year in 2000..2022 {
      for cat in [ Passing, Rushing, Receiving, Scrimmage, Defense, Kicking, Punting, Returns, Scoring ].iter() {
         fetch_stats_and_add_to_table( year, cat ).await?;
      }
   }

   Ok(())
}