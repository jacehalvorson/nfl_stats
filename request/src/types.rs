// Custom types
#[derive(Debug)]
pub struct DynamoDBItem {
   pub id: String,
   pub attributes: Vec<String>,
   pub players: Vec<Vec<String>>
}

#[derive(Debug)]
pub enum Category {
   Passing,
   Rushing,
   Receiving,
   Defense,
   Kicking,
   Punting,
   Returns,
   Scoring
}