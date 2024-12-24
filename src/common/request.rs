use serde::Serialize;
use validator::{ValidationErrors};

#[derive(Serialize)]
pub struct ErrorResponse {
    pub(crate) is_error: bool,
    pub(crate) message: String,
}

pub fn convert_validate_error_to_response(e: ValidationErrors) -> ErrorResponse {
    let missing_fields = e.field_errors();
    let mut message = "validate error in field: ".to_string();

    for (field, _error) in missing_fields {
        message.push_str(format!("`{}` ", field).as_str());
    }

    ErrorResponse {
        is_error: true,
        message,
    }
}

