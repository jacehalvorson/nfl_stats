use nflstatsrequest::utils::get_page;

#[tokio::main]
async fn main() {
   get_page().await.unwrap();
}