use crate::gql::Product;
use async_graphql::*;

#[derive(SimpleObject)]
pub struct Offer {
  pub id: ID,
  pub price: i32,
  pub product: Product,
}
