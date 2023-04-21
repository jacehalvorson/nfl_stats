#[allow(dead_code, unused_variables, unused_macros)]

pub mod json;
pub mod put_item;

use std::env::args;
use crate::put_item::fetch_and_add_to_table;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
   let args: Vec<String> = args().collect();
   if args.len() != 3 {
      println!( "Usage: cargo run --release <year> <category>" );
      return Ok(());
   }

   let year = args[1].parse::<i32>().unwrap();
   let category = &args[2];
   fetch_and_add_to_table( year, category ).await?;

   Ok(())
}