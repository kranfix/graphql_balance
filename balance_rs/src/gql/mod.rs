pub mod customer;
pub mod offer;
pub mod product;
pub mod scalars;

use async_graphql::*;
use customer::{CustomerMutation, CustomerQuery};
use offer::Offer;
use product::Product;
use scalars::GqlUrl;

#[derive(MergedObject, Default)]
pub struct QueryRoot(CustomerQuery);

#[derive(MergedObject, Default)]
pub struct MutationRoot(CustomerMutation);
