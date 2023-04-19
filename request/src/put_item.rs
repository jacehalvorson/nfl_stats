pub async fn list_tables(client: &Client) -> Result<Vec<String>, Error> {
   let paginator = client.list_tables().into_paginator().items().send();
   let table_names = paginator.collect::<Result<Vec<_>, _>>().await?;

   println!("Tables:");

   for name in &table_names {
       println!("  {}", name);
   }

   println!("Found {} tables", table_names.len());
   Ok(table_names)
}
