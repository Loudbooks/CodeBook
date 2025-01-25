use crate::database::aws_service::AWSService;
use crate::models::paste::Paste;
use crate::database::mongodb_service::MongoService;
use crate::utils::iputils::IPUtils;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

pub async fn upload_handler(
    aws_service: web::Data<Arc<AWSService>>,
    mongo_service: web::Data<Arc<MongoService>>,
    req: HttpRequest,
    body: String,
) -> impl Responder {
    let expires =
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
            + 604800;

    let since_the_epoch = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64;
    
    let language = req
        .headers()
        .get("Language")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("plaintext");

    let ip = match IPUtils::get_ip_from_request(&req) {
        Some(ip) => ip,
        None => return HttpResponse::BadRequest().body("Failed to get IP"),
    };

    let file_id = generate_random_string(5);

    let paste = Paste {
        id: file_id.clone(),
        created: since_the_epoch,
        creator_ip: ip.clone(),
        expires_at: expires,
        language: language.to_string(),
    };

    if let Err(e) = aws_service.put_file(&file_id, body.as_ref()).await {
        return HttpResponse::InternalServerError().body(format!("Failed to upload file: {:?}", e));
    }
    if let Err(e) = mongo_service.put_paste(paste).await {
        return HttpResponse::InternalServerError().body(format!("Failed to save to database: {:?}", e));
    }

    mongo_service.increment_requests(&ip).await.expect("Failed to increment requests");

    let host_domain = req
        .headers()
        .get("X-Domain-Name")
        .and_then(|v| v.to_str().ok());
    let response_body = if let Some(domain) = host_domain {
        format!("https://{}/p/{}", domain, file_id)
    } else {
        file_id.clone()
    };

    HttpResponse::Ok().body(response_body)
}

fn generate_random_string(length: usize) -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()
}
