use shopify_function::prelude::*;
use std::process;

pub mod cart_validations_generate_run;

#[typegen("schema.graphql")]
mod schema {
    #[query(
        "src/cart_validations_generate_run.graphql",
        custom_scalar_overrides = {
            "Input.validation.metafield.jsonValue" => super::cart_validations_generate_run::Configuration
        }
    )]
    pub mod run {}
}

fn main() {
    eprintln!("Please invoke a named export.");
    process::exit(1);
}
