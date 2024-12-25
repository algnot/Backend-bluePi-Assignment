use actix_web::{web, HttpResponse, Responder};
use base64::Engine;
use crate::common::request::ErrorResponse;
use crate::repository::uploader::Uploader;
use base64::engine::general_purpose::STANDARD;
use futures_util::stream;

pub async fn get_image(path: web::Path<(String,)>) -> impl Responder {
    let image_id = path.into_inner().0;

    let content = Uploader::new().get_upload_by_id(&image_id);

    match content {
        Some(uploader) => {
            let base64content = uploader.body;

            match STANDARD.decode(base64content) {
                Ok(decoded_data) => {
                    let byte_stream = stream::once(async { Ok(web::Bytes::from(decoded_data)) });

                    HttpResponse::Ok()
                        .content_type("image/png")
                        .streaming::<_, std::io::Error>(byte_stream)
                }
                Err(_) => {
                    HttpResponse::BadRequest().json({
                        ErrorResponse {
                            is_error: true,
                            message: "cannot get image or deleted".to_string(),
                        }
                    })
                }
            }
        }
        None => {
            HttpResponse::BadRequest().json({
                ErrorResponse {
                    is_error: true,
                    message: "cannot get image or deleted".to_string(),
                }
            })
        }
    }
}