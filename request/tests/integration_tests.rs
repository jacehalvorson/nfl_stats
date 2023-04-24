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
   assert_eq!(filter_stat("NWE".to_string(), 3, Passing), "NE");
   assert_eq!(filter_stat("KAN".to_string(), 3, Passing), "KC");
   assert_eq!(filter_stat("GNB".to_string(), 3, Passing), "GB");
   assert_eq!(filter_stat("".to_string(), 3, Passing), "");
   assert_eq!(filter_stat("MIN".to_string(), 3, Passing), "MIN");

   // QB record
   assert_eq!(filter_stat("4-2-0".to_string(), 7, Passing), "4-2-0");
   assert_eq!(filter_stat("".to_string(), 7, Passing), "0-0-0");

   // QB yards per carry
   assert_eq!(filter_stat("9.0".to_string(), 20, Passing), "9.0");
   assert_eq!(filter_stat("".to_string(), 20, Passing), "--");
}

#[tokio::test]
async fn test_fetch_no_empty() {
   const 2022_passing_stats = get_stats( 2022, "passing" ).await.unwrap();

   // Make sure there are no empty strings in the stats
   for player in 2022_passing_stats.players {
      for stat in player {
         assert_ne!(stat, "");
      }
   }
}