use aws_sdk_dynamodb::{ types::AttributeValue, Client, Error };
use aws_config::meta::region::RegionProviderChain;
use crate::get_stats::get_stats;
use crate::types::DynamoDBItem;

pub async fn fetch_and_add_to_table( year : i32, category: &str ) -> Result<(), Error> {
    // Initialize the client
    let region_provider = RegionProviderChain::default_provider().or_else("us-east-1");
    let shared_config = aws_config::from_env().region( region_provider ).load().await;
    let client = Client::new( &shared_config );

    // Fetch the requested stats
    let result : DynamoDBItem = get_stats( year, category ).await.unwrap();
    println!( "{}\n{}\n{:?}\n", result.id, result.attributes[0], result.players[0] );

    // Define values with AttributeValue types
    let id = AttributeValue::S( result.id );
    let attributes = AttributeValue::L( result.attributes.iter().map( |x| AttributeValue::S( x.to_string() ) ).collect() );
    let players = AttributeValue::L( result.players.iter().map( |x| AttributeValue::L( x.iter().map( |y| AttributeValue::S( y.to_string() ) ).collect() ) ).collect() );

    // Put the stats into DynamoDB table
    let request = client
        .put_item()
        .table_name( "NFLStats-main" )
        .item( "id", id )
        .item( "attributes", attributes )
        .item( "players", players );

    request.send().await?;

    Ok(())
}