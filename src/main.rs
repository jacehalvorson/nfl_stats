include!( r"get_stats_and_write_to_html.rs" );
#[allow(dead_code, unused_variables, unused_macros)]

#[tokio::main]
async fn main() {
   println!();
   let url = format!( "https://www.pro-football-reference.com/years/2022/{}.htm", "passing" );
   get_stats_and_write_to_html( &url ).await.unwrap();
   println!();
}