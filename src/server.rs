use anyhow::Result;
use core::str;
use std::thread::sleep;
use std::time::Duration;
use esp_idf_svc::{
    http::server::{Configuration, EspHttpServer},
};
use esp_idf_svc::http::{Method};
use esp_idf_svc::io::Write;
use const_format::formatcp;
use crate::camera;

// Arbitrary boundary identifier
const PART_BOUNDARY: &str = "123456789000000000000987654321";
// Example: https://wiki.tcl-lang.org/page/multipart%2Fx-mixed-replace#:~:text=multipart%2Fx%2Dmixed%2Dreplace%20is%20an%20HTTP%20content%20type,content%20to%20the%20web%20browser.
const STREAM_CONTENT_TYPE: &str = formatcp!("multipart/x-mixed-replace;boundary={}", PART_BOUNDARY);
const STREAM_BOUNDARY: &str = formatcp!("\r\n--{}\r\n", PART_BOUNDARY);

pub fn serve(camera: camera::Camera<'static>) -> Result<EspHttpServer<'static>> {

    // Set the HTTP server
    let mut server = EspHttpServer::new(&Configuration::default())?;
    server.fn_handler("/stream", Method::Get, move |request| {
        
        let mut response =request.into_response(
            200,
            Some("OK"),
            &[("Content-Type", STREAM_CONTENT_TYPE)],
        )?;

        response.write_all(STREAM_BOUNDARY.as_bytes())?;

        loop {
            let jpg = camera.get_framebuffer().unwrap().data();
            response.write_all(format!("Content-Type: image/jpeg\r\nContent-Length: {}\r\n\r\n", jpg.len()).as_bytes())?;
            response.write_all(jpg)?;
            response.flush()?;
            response.write_all(STREAM_BOUNDARY.as_bytes())?;
        }
    })?;

    println!("Server awaiting connection");
    return Ok(server);
}