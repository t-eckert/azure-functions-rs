use crate::rpc::TypedData;
use serde_derive::Deserialize;
use std::collections::HashMap;

/// Represents the Durable Functions activity context binding.
///
/// The following binding attributes are supported:
///
/// | Name       | Description                                                      |
/// |------------|------------------------------------------------------------------|
/// | `name`     | The name of the parameter being bound.                           |
/// | `activity` | The name of the activity.  Defaults to the name of the function. |
///
/// # Examples
///
/// 
/// ``` rust
/// use azure_functions::{bindings::DurableActivityContext, func};
///
///#[func]
///pub fn say_hello(context: DurableActivityContext) {
///    context.set_output(format!(
///        "Hello {}!",
///        context
///            .input()
///            .as_str()
///            .expect("expected a string input")
///    ));
///}
/// ```
/// 
/// 
/// TODO: IMPLEMENT
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DurableActivityContext {}

impl DurableActivityContext {
    pub fn instance_id(&self) -> &str {
        "placeholder"
    }

    pub fn input(&self) -> serde_json::Value {
        
    }

    pub fn set_output<O>(&mut self, output: O) where O: Into<serde_json::Value> {

    }


    #[doc(hidden)]
    pub fn new(data: TypedData, metadata: HashMap<String, TypedData>) -> Self {
        println!("{:#?}", data);
        println!("{:#?}", metadata);
        DurableActivityContext {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rpc::typed_data::Data;

    #[test]
    fn it_constructs() {
        let data = TypedData {
            data: Some(Data::String(r#"{ }"#.to_owned())),
        };

        let _ = DurableActivityContext::new(data, HashMap::new());

        // TODO: implement
    }
}
