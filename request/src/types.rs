// Custom types
#[derive(Debug)]
pub struct DynamoDBItem {
   pub id: String,
   pub attributes: Vec<String>,
   pub players: Vec<Vec<String>>
}

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord)]
pub enum Category {
   Passing,
   Rushing,
   Receiving,
   Scrimmage,
   Defense,
   Kicking,
   Punting,
   Returns,
   Scoring
}