#[allow(dead_code, unused_variables, unused_macros)]

pub mod get_stats;
pub mod put_item;
pub mod types;

use std::env::args;
use crate::put_item::fetch_and_add_to_table;
use crate::get_stats::filter_stat;
use crate::types::Category::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
   let args: Vec<String> = args().collect();
   if args.len() != 3 {
      println!( "Usage: cargo run --release <year> <category>" );
      return Ok(());
   }

   let year = args[1].parse::<i32>().unwrap();
   let category = &args[2];
   // fetch_and_add_to_table( year, category ).await?;
   println!( "{}", filter_stat( "NWE".to_string(), 3, Passing ) );
   println!( "{}", filter_stat( "KAN".to_string(), 3, Passing ) );
   println!( "{}", filter_stat( "GNB".to_string(), 3, Passing ) );
   println!( "{}", filter_stat( "".to_string(), 3, Passing ) );
   println!( "{}", filter_stat( "MIN".to_string(), 3, Passing ) );

   println!( "{}", filter_stat( "4-2-0".to_string(), 7, Passing ) );
   println!( "{}", filter_stat( "".to_string(), 7, Passing ) );
   println!( "{}", filter_stat( "0-0-0".to_string(), 7, Passing ) );
   
   println!( "{}", filter_stat( "9.0".to_string(), 20, Passing ) );
   println!( "{}", filter_stat( "".to_string(), 20, Passing ) );

   Ok(())
}