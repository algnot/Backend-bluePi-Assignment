use actix_multipart::Multipart;
use actix_web::{web, HttpResponse, Responder};
use base64::Engine;
use log::{info, warn};
use serde::Serialize;
use base64::engine::general_purpose::STANDARD;
use futures_util::stream::StreamExt;
use crate::common::request::ErrorResponse;
use crate::repository::uploader::Uploader;

#[derive(Serialize)]
pub struct UploadImageResponse {
    id: String
}

pub async fn upload_image(mut payload: Multipart) -> impl Responder {
    let mut base64_image = String::new();

    while let Some(item) = payload.next().await {
        if let Ok(mut field) = item {
            let content_disposition = field.content_disposition().unwrap();
            if let Some(filename) = content_disposition.get_filename() {
                info!("Received file: {}", filename);

                let mut bytes = web::BytesMut::new();
                while let Some(chunk) = field.next().await {
                    match chunk {
                        Ok(data) => bytes.extend_from_slice(&data),
                        Err(e) => {
                            warn!("Error while reading file: {}", e);
                            return HttpResponse::InternalServerError().finish();
                        }
                    }
                }

                base64_image = STANDARD.encode(&bytes);
                break;
            }
        }
    }

    let upload_file = Uploader::new().upload(&base64_image);

    match upload_file {
        Some(file) => {
            HttpResponse::Ok().json({
                UploadImageResponse {
                    id: file.id.to_string(),
                }
            })
        }
        None => {
            HttpResponse::InternalServerError().json({
                ErrorResponse {
                    is_error: true,
                    message: "cannot upload file try again".to_string(),
                }
            })
        }
    }
}
