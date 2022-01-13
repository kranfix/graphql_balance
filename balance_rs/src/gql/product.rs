use crate::gql::GqlUrl;
use async_graphql::*;

#[derive(SimpleObject)]
pub struct Product {
  pub id: ID,
  pub name: String,
  pub description: String,
  pub image: GqlUrl,
}
