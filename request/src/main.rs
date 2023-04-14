#[allow(dead_code, unused_variables, unused_macros)]

pub mod json;

use std::env::args;
use crate::json::get_stats_and_write_to_json;

fn main() {
   let args: Vec<String> = args().collect();
   if args.len() != 3 {
      println!( "Usage: cargo run --release <year> <category>" );
      return;
   }

   // TODO Check args[1] and args[2] for valid values

   let url = format!( "https://www.pro-football-reference.com/years/{year}/{category}.htm", year=args[1], category=args[2] );
   get_stats_and_write_to_json( &url ).unwrap();
}