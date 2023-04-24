#[path = "../src/get_stats.rs"]
mod get_stats;

#[path = "../src/put_item.rs"]
mod put_item;

#[path = "../src/types.rs"]
mod types;

use crate::get_stats::{filter_stat, get_stats};
use crate::types::Category::*;

#[tokio::test]
async fn test_filter_stat() {
   // Team names
   assert_eq!( filter_stat( "NWE".to_string(), "Tm".to_string() ), "NE" );
   assert_eq!( filter_stat( "KAN".to_string(), "Tm".to_string() ), "KC" );
   assert_eq!( filter_stat( "GNB".to_string(), "Tm".to_string() ), "GB" );
   assert_eq!( filter_stat( "".to_string(), "Tm".to_string() ), "" );
   assert_eq!( filter_stat( "MIN".to_string(), "Tm".to_string() ), "MIN" );

   // QB record
   assert_eq!( filter_stat( "4-2-0".to_string(), "QBrec".to_string() ), "4-2-0" );
   assert_eq!( filter_stat( "".to_string(), "QBrec".to_string() ), "0-0-0" );

   // QB yards per carry
   assert_eq!( filter_stat( "9.0".to_string(), "Y/C".to_string() ), "9.0" );
   assert_eq!( filter_stat( "".to_string(), "Y/C".to_string() ), "--" );
}

#[tokio::test]
async fn test_fetch_no_empty() {
   let passing_stats_2022 = get_stats( 2022, &Passing ).await.unwrap();

   // Make sure there are no empty strings in the stats
   for player in passing_stats_2022.players {
      for stat in player {
         assert_ne!(stat, "");
      }
   }
}