use warp::{Filter, Rejection, Reply};
use std::path::PathBuf;
use std::fs;
use bytes::Bytes;
use futures::StreamExt;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct DetectionResult {
    frame: u32,
    objects: Vec<Object>,
}

#[derive(Serialize, Deserialize)]
struct Object {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    class: String,
    confidence: f32,
}

pub async fn start_server() -> Result<(), Box<dyn std::error::Error>> {
    let static_files = warp::path("static")
        .and(warp::fs::dir("./static"));

    let upload_route = warp::path("upload")
        .and(warp::post())
        .and(warp::multipart::form().max_length(50_000_000)) // 50MB limit
        .and_then(handle_upload);

    let get_video = warp::path("processed")
        .and(warp::path::param())
        .and(warp::fs::file("./processed/output.mp4"));

    let routes = static_files
        .or(upload_route)
        .or(get_video)
        .or(warp::path::end().map(|| warp::redirect::temporary("/static/index.html")));

    println!("Server started at http://localhost:8080");
    warp::serve(routes).run(([0, 0, 0, 0], 8080)).await;

    Ok(())
}

async fn handle_upload(form: warp::multipart::FormData) -> Result<impl Reply, Rejection> {
    let parts: Vec<warp::multipart::Part> = form.try_collect().await.map_err(|e| {
        eprintln!("upload error: {}", e);
        warp::reject::reject()
    })?;

    for p in parts {
        if p.name() == "video" {
            let content_type = p.content_type().map(|m| m.to_string());
            if content_type.as_deref().unwrap_or("").starts_with("video/") {
                let file_path = PathBuf::from("uploads/video.mp4");
                fs::create_dir_all("uploads").map_err(|e| {
                    eprintln!("create dir error: {}", e);
                    warp::reject::reject()
                })?;

                let data = p
                    .stream()
                    .try_fold(Vec::new(), |mut vec, data| async move {
                        vec.extend_from_slice(&data);
                        Ok(vec)
                    })
                    .await
                    .map_err(|e| {
                        eprintln!("reading file error: {}", e);
                        warp::reject::reject()
                    })?;

                fs::write(&file_path, data).map_err(|e| {
                    eprintln!("write file error: {}", e);
                    warp::reject::reject()
                })?;

                // In a real app, you would process the video here
                // and return the processed video path
                return Ok(warp::reply::json(&"Processing started"));
            }
        }
    }

    Err(warp::reject::reject())
}
