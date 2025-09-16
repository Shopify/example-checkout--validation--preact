use super::schema;
use shopify_function::prelude::*;
use shopify_function::Result;
use std::collections::HashMap;

#[derive(Deserialize, Default, PartialEq)]
pub struct Configuration {
    limits: HashMap<String, i32>,
}

#[shopify_function]
fn run(input: schema::run::Input) -> Result<schema::CartValidationsGenerateRunResult> {
    let mut operations = Vec::new();
    let mut errors = Vec::new();

    let configuration = if let Some(metafield) = input.validation().metafield() {
        metafield.json_value()
    } else {
        return Ok(schema::CartValidationsGenerateRunResult { operations: vec![] });
    };

    input.cart().lines().iter().for_each(|line| {
        let quantity = line.quantity();
        match &line.merchandise() {
            schema::run::input::cart::lines::Merchandise::ProductVariant(variant) => {
                let limit = configuration.limits.get(variant.id()).unwrap_or(&i32::MAX);
                let product_name = variant.product().title();

                // Check item quantity in the cart against the configured limit
                if quantity > limit {
                    errors.push(schema::ValidationError {
                        message: format!(
                            "Orders are limited to a maximum of {} of {}",
                            limit, product_name
                        ),
                        target: "cart".to_owned(),
                    });
                }
            }
            _ => {}
        };
    });

    let operation = schema::ValidationAddOperation { errors };
    operations.push(schema::Operation::ValidationAdd(operation));
    Ok(schema::CartValidationsGenerateRunResult { operations })
}
