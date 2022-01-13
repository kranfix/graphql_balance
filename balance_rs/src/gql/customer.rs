use crate::gql::{GqlUrl, Offer, Product};
use async_graphql::*;

#[derive(SimpleObject)]
#[graphql(complex)] //
pub struct Customer {
  pub id: ID,
  pub name: String,
  pub balance: i32,
}

#[ComplexObject]
impl Customer {
  pub async fn offers(&self) -> Vec<Offer> {
    offers()
  }
}

#[derive(Default)]
pub struct CustomerQuery;

#[Object]
impl CustomerQuery {
  pub async fn viewer(&self) -> Customer {
    Customer {
      id: "cccc3f48-dd2c-43ba-b8de-8945e7ababab".into(),
      name: "Jerry Smith".to_owned(),
      balance: 995000,
    }
  }
}

#[derive(Default)]
pub struct CustomerMutation;

#[Object]
impl CustomerMutation {
  pub async fn purchase(&self) -> PurchaseMutationResponse {
    PurchaseMutationResponse {
      success: true,
      error_message: "Purchase successful".to_owned(),
      customer: Customer {
        id: "cccc3f48-dd2c-43ba-b8de-8945e7ababab".into(),
        name: "Jerry Smith".to_owned(),
        balance: 995000,
      },
    }
  }
}

#[derive(SimpleObject)]
pub struct PurchaseMutationResponse {
  pub success: bool,
  pub error_message: String,
  pub customer: Customer,
}

pub fn offers() -> Vec<Offer> {
  return vec![
    Offer {
      id: "offer/portal-gun".into(),
      price: 5000,
      product: Product {
        id: ID("product/portal-gun".to_owned()),
        name: "Portal Gun".to_owned(),
        description: "The Portal Gun is a gadget that allows the user(s) to travel between different universes/dimensions/realities.".to_owned(),
        image: GqlUrl("https://vignette.wikia.nocookie.net/rickandmorty/images/5/55/Portal_gun.png/revision/latest/scale-to-width-down/310?cb=20140509065310".to_owned()),
      }
    },
    Offer {
      id: "offer/microverse-battery".into(),
      price: 5507,
      product: Product {
        id: ID("product/microverse-battery".to_owned()),
        name: "Microverse Battery".to_owned(),
        description: "The Microverse Battery contains a miniature universe with a planet inhabited by intelligent life.".to_owned(),
        image: GqlUrl("https://vignette.wikia.nocookie.net/rickandmorty/images/8/86/Microverse_Battery.png/revision/latest/scale-to-width-down/310?cb=20160910010946".to_owned()),
      }
    },
    Offer {
      id: "offer/mr-meeseeks-box".into(),
      price: 999999999,
      product: Product {
        id: "product/mr-meeseeks-box".into(),
        name: "Mr. Meeseeks Box".to_owned(),
        description: "The Mr. Meeseeks Box is a gadget that creates a Mr. Meeseeks for the purpose of completing one given objective.".to_owned(),
        image: GqlUrl("https://vignette.wikia.nocookie.net/rickandmorty/images/f/f7/Mr._Meeseeks_Box.png/revision/latest/scale-to-width-down/310?cb=20160909153718".to_owned()),
      }
    },
    Offer {
      id: ID("offer/cognition-amplifier".to_owned()),
      price: 1000000,
      product: Product {
        id: ID("product/cognition-amplifier".to_owned()),
        name: "Cognition Amplifier".to_owned(),
        description: "The cognition amplifier makes Snuffles smart.".to_owned(),
        image: GqlUrl("https://vignette.wikia.nocookie.net/rickandmorty/images/2/27/Cognition_Amplifier.png/revision/latest/scale-to-width-down/180?cb=20140604001816".to_owned()),
      }
    },
  ];
}
