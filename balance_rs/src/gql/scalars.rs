use async_graphql::*;

pub struct GqlUrl(pub String);

#[Scalar]
impl ScalarType for GqlUrl {
  fn parse(value: Value) -> InputValueResult<Self> {
    if let Value::String(value) = &value {
      // Parse the integer value
      Ok(GqlUrl(value.to_owned()))
    } else {
      // If the type does not match
      Err(InputValueError::expected_type(value))
    }
  }

  fn to_value(&self) -> Value {
    Value::String(self.0.to_owned())
  }
}
