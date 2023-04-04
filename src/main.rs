use nflstatsrequest::write_stats_to_json::write_stats_to_json;

#[tokio::main]
async fn main() {
   println!();
   let url = "https://www.pro-football-reference.com/years/2022/passing.htm";
   write_stats_to_json( url ).await.unwrap();
   println!();
}