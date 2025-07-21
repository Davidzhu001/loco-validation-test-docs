use loco_rs::{controller::extractor::validate::*, prelude::*};

use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct DataParams {
    #[validate(length(min = 5, message = "custom message"))]
    pub name: String,
    #[validate(email)]
    pub email: String,
}

pub async fn json_validate(JsonValidate(params): JsonValidate<DataParams>) -> Result<Response> {
    format::json(params)
}

pub async fn json_validate_with_message(
    JsonValidateWithMessage(params): JsonValidateWithMessage<DataParams>,
) -> Result<Response> {
    format::json(params)
}

// pub async fn query_validate(QueryValidate(params): QueryValidate<DataParams>) -> Result<Response> {
//     format::json(params)
// }

// pub async fn query_validate_with_message(
//     QueryValidateWithMessage(params): QueryValidateWithMessage<DataParams>,
// ) -> Result<Response> {
//     format::json(params)
// }

pub async fn form_validate(FormValidate(params): FormValidate<DataParams>) -> Result<Response> {
    format::json(params)
}

pub async fn form_validate_with_message(
    FormValidateWithMessage(params): FormValidateWithMessage<DataParams>,
) -> Result<Response> {
    format::json(params)
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("/backend/validations")
        .add("/json_validate", post(json_validate))
        .add(
            "/json_validate_with_message",
            post(json_validate_with_message),
        )
        // .add("/query_validate", get(query_validate))
        // .add(
        //     "/query_validate_with_message",
        //     get(query_validate_with_message),
        // )
        .add("/form_validate", post(form_validate))
        .add(
            "/form_validate_with_message",
            post(form_validate_with_message),
        )
}
