# loco-validation-test-reports

## Env

```rs
loco version: // github master 2025-07-21
```
### test file
```rs
use loco_rs::{controller::extractor::validate::*, prelude::*};

use serde::{Deserialize, Serialize};
use validator::Validate;

// VALIDATOR
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
        .add("/form_validate", post(form_validate))
        .add(
            "/form_validate_with_message",
            post(form_validate_with_message),
        )
}

```
### Validation Defination
```rs
#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct DataParams {
    #[validate(length(min = 5, message = "custom message"))]
    pub name: String,
    #[validate(email)]
    pub email: String,
}
```

| Test Case No. | Validation Function           | Content-Type                       | Payload         | Actual Status Code | Expected Status | Log Information                                                                 | Notes                                                                |
|---------------|------------------------------|------------------------------------|-----------------|--------------------|-----------------|---------------------------------------------------------------------------------|----------------------------------------------------------------------|
| TC01          | json_validate                | application/json                  | valid_payload   | 200                | ✅Expected       | None                                                                            | Valid JSON payload, meets validation rules.                           |
| TC02          | json_validate                | application/json                  | invalid_payload | 400                | ✅Expected       | None                                                                            | Invalid JSON payload, `name` is empty, `email` is invalid.            |
| TC03          | json_validate                | application/json                  | {}              | 422                | ❌Unexpected     | ValidationErrors({"name": "custom message"})                                     | Empty object, missing required fields, expected 400 but returned 422. |
| TC04          | json_validate                | application/json                  | Null            | 400                | ✅Expected       | Failed to parse JSON: EOF while parsing a value at line 1 column 0              | Empty request body, JSON parsing failed.                              |
| TC05          | json_validate                | application/xml                   | valid_payload   | 415                | ✅Expected       | Expected `Content-Type: application/json`                                       | Unsupported Content-Type.                                             |
| TC06          | json_validate                | application/xml                   | invalid_payload | 415                | ✅Expected       | Expected `Content-Type: application/json`                                       | Unsupported Content-Type.                                             |
| TC07          | json_validate                | application/xml                   | {}              | 415                | ✅Expected       | Expected `Content-Type: application/json`                                       | Unsupported Content-Type.                                             |
| TC08          | json_validate                | application/xml                   | Null            | 415                | ✅Expected       | Expected `Content-Type: application/json`                                       | Unsupported Content-Type.                                             |
| TC09          | json_validate                | application/x-www-form-urlencoded | valid_payload   | 415                | ✅Expected       | Expected `Content-Type: application/json`                                       | Unsupported Content-Type.                                             |
| TC10          | json_validate                | application/x-www-form-urlencoded | invalid_payload | 415                | ✅Expected       | Expected `Content-Type: application/json`                                       | Unsupported Content-Type.                                             |
| TC11          | json_validate                | application/x-www-form-urlencoded | {}              | 415                | ✅Expected       | Expected `Content-Type: application/json`                                       | Unsupported Content-Type.                                             |
| TC12          | json_validate                | application/x-www-form-urlencoded | Null            | 415                | ✅Expected       | Expected `Content-Type: application/json`                                       | Unsupported Content-Type.                                             |
| TC13          | json_validate_with_message   | application/json                  | valid_payload   | 200                | ✅Expected       | None                                                                            | Valid JSON payload, meets validation rules.                           |
| TC14          | json_validate_with_message   | application/json                  | invalid_payload | 400                | ✅Expected       | None                                                                            | Invalid JSON payload, `name` is empty, `email` is invalid.            |
| TC15          | json_validate_with_message   | application/json                  | {}              | 422                | ❌Unexpected     | ValidationErrors({"name": "custom message"})                                     | Empty object, missing required fields, expected 400 but returned 422. |
| TC16          | json_validate_with_message   | application/json                  | Null            | 400                | ✅Expected       | Failed to parse JSON: EOF while parsing a value at line 1 column 0              | Empty request body, JSON parsing failed.                              |
| TC17          | json_validate_with_message   | application/xml                   | valid_payload   | 415                | ✅Expected       | Expected `Content-Type: application/json`                                       | Unsupported Content-Type.                                             |
| TC18          | json_validate_with_message   | application/xml                   | invalid_payload | 415                | ✅Expected       | Expected `Content-Type: application/json`                                       | Unsupported Content-Type.                                             |
| TC19          | json_validate_with_message   | application/xml                   | {}              | 415                | ✅Expected       | Expected `Content-Type: application/json`                                       | Unsupported Content-Type.                                             |
| TC20          | json_validate_with_message   | application/xml                   | Null            | 415                | ✅Expected       | Expected `Content-Type: application/json`                                       | Unsupported Content-Type.                                             |
| TC21          | json_validate_with_message   | application/x-www-form-urlencoded | valid_payload   | 415                | ✅Expected       | Expected `Content-Type: application/json`                                       | Unsupported Content-Type.                                             |
| TC22          | json_validate_with_message   | application/x-www-form-urlencoded | invalid_payload | 415                | ✅Expected       | Expected `Content-Type: application/json`                                       | Unsupported Content-Type.                                             |
| TC23          | json_validate_with_message   | application/x-www-form-urlencoded | {}              | 415                | ✅Expected       | Expected `Content-Type: application/json`                                       | Unsupported Content-Type.                                             |
| TC24          | json_validate_with_message   | application/x-www-form-urlencoded | Null            | 415                | ✅Expected       | Expected `Content-Type: application/json`                                       | Unsupported Content-Type.                                             |
| TC25          | form_validate                | application/x-www-form-urlencoded | valid_payload   | 200                | ✅Expected       | None                                                                            | Valid form payload, meets validation rules.                           |
| TC26          | form_validate                | application/x-www-form-urlencoded | invalid_payload | 400                | ✅Expected       | None                                                                            | Invalid form payload, `name` is empty, `email` is invalid.            |
| TC27          | form_validate                | application/x-www-form-urlencoded | null            | 500                | 💣❌Unexpected   | Failed to deserialize form body: missing field `email`                          | Empty request body, form parsing failed, expected 400 but returned 500. |
| TC28          | form_validate                | application/x-www-form-urlencoded | {}              | 500                | 💣❌Unexpected   | Failed to deserialize form body: missing field `name`                           | Empty object, form parsing failed, expected 400 but returned 500.     |
| TC29          | form_validate                | application/json                  | valid_payload   | 500                | 💣❌Unexpected   | Expected `Content-Type: application/x-www-form-urlencoded`                      | Unsupported Content-Type, expected 415.                               |
| TC30          | form_validate                | application/json                  | invalid_payload | 500                | 💣❌Unexpected   | Expected `Content-Type: application/x-www-form-urlencoded`                      | Unsupported Content-Type, expected 415.                               |
| TC31          | form_validate                | application/json                  | {}              | 500                | 💣❌Unexpected   | Expected `Content-Type: application/x-www-form-urlencoded`                      | Unsupported Content-Type, expected 415.                               |
| TC32          | form_validate                | application/json                  | Null            | 500                | 💣❌Unexpected   | Expected `Content-Type: application/x-www-form-urlencoded`                      | Unsupported Content-Type, expected 415.                               |
| TC33          | form_validate_with_message   | application/x-www-form-urlencoded | valid_payload   | 200                | ✅Expected       | None                                                                            | Valid form payload, meets validation rules.                           |
| TC34          | form_validate_with_message   | application/x-www-form-urlencoded | invalid_payload | 400                | ✅Expected       | None                                                                            | Invalid form payload, `name` is empty, `email` is invalid.            |
| TC35          | form_validate_with_message   | application/x-www-form-urlencoded | null            | 500                | 💣❌Unexpected   | Failed to deserialize form body: missing field `email`                          | Empty request body, form parsing failed, expected 400 but returned 500. |
| TC36          | form_validate_with_message   | application/x-www-form-urlencoded | {}              | 500                | 💣❌Unexpected   | Failed to deserialize form body: missing field `name`                           | Empty object, form parsing failed, expected 400 but returned 500.     |
| TC37          | form_validate_with_message   | application/json                  | valid_payload   | 500                | 💣❌Unexpected   | Expected `Content-Type: application/x-www-form-urlencoded`                      | Unsupported Content-Type, expected 415.                               |
| TC38          | form_validate_with_message   | application/json                  | invalid_payload | 500                | 💣❌Unexpected   | Expected `Content-Type: application/x-www-form-urlencoded`                      | Unsupported Content-Type, expected 415.                               |
| TC39          | form_validate_with_message   | application/json                  | {}              | 500                | 💣❌Unexpected   | Expected `Content-Type: application/x-www-form-urlencoded`                      | Unsupported Content-Type, expected 415.                               |
| TC40          | form_validate_with_message   | application/json                  | Null            | 500                | 💣❌Unexpected   | Expected `Content-Type: application/x-www-form-urlencoded`                      | Unsupported Content-Type, expected 415.                               |


## Details
[Uploading ValitionTest.csv…]()
