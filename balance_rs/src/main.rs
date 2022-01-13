mod gql;

use crate::gql::{MutationRoot, QueryRoot};
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql::{EmptySubscription, Schema};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::extract::Extension;
use axum::response::{self, IntoResponse};
use axum::routing::get;
use axum::{AddExtensionLayer, Router, Server};

type StarWarsSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

async fn graphql_handler(
  schema: Extension<StarWarsSchema>,
  req: GraphQLRequest,
) -> GraphQLResponse {
  schema.execute(req.into_inner()).await.into()
}

async fn graphql_playground() -> impl IntoResponse {
  response::Html(playground_source(GraphQLPlaygroundConfig::new("/")))
}

#[tokio::main]
async fn main() {
  let schema = Schema::new(
    QueryRoot::default(), //
    MutationRoot::default(),
    EmptySubscription,
  );
  //.data(StarWars::new())
  //.finish();

  let app = Router::new()
    .route("/", get(graphql_playground).post(graphql_handler))
    .layer(AddExtensionLayer::new(schema));

  println!("Playground: http://localhost:8000");

  Server::bind(&"0.0.0.0:8000".parse().unwrap())
    .serve(app.into_make_service())
    .await
    .unwrap();
}
