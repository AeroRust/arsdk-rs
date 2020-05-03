use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::{
    io::{Read, Write},
    net::{SocketAddr, TcpStream},
};

#[derive(Serialize)]
pub(crate) struct Request {
    controller_name: String,
    controller_type: String,
    d2c_port: u16,
    #[serde(skip_serializing_if = "Option::is_some")]
    arstream2_client_stream_port: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_some")]
    arstream2_client_control_port: Option<u16>,
}

#[derive(Deserialize, Debug)]
/// Request: "{\"controller_name\":\"arsdk-rs\",\"controller_type\":\"computer\",\"d2c_port\":43210}"
/// Response: "{ \"status\": 0, \"c2d_port\": 54321, \"c2d_update_port\": 51, \"c2d_user_port\": 21, \"qos_mode\": 0, \"arstream2_server_stream_port\": 5004, \"arstream2_server_control_port\": 5005 }\u{0}"
/// `\u{0}` causes issues, but for now we `trim_end_matches`
/// Error: trailing characters at line 1 column 171
pub(crate) struct Response {
    #[serde(default)]
    pub arstream_fragment_maximum_number: Option<u8>,
    #[serde(default)]
    pub arstream_fragment_size: Option<u16>,
    #[serde(default)]
    pub arstream_max_ack_interval: Option<i8>,
    pub arstream2_server_stream_port: u16,
    pub arstream2_server_control_port: u16,
    pub c2d_port: u16,
    pub c2d_update_port: u16,
    pub c2d_user_port: u16,
    pub status: i8,
    // @TODO: qos_mode: bool maybe?!
}

pub(crate) fn perform_handshake(target: SocketAddr, d2c_port: u16) -> Result<Response> {
    let request = Request {
        controller_name: "arsdk-rs".to_string(),
        controller_type: "computer".to_string(),
        d2c_port,
        arstream2_client_stream_port: Some(44445),
        arstream2_client_control_port: Some(44446),
    };

    println!("connecting controller {}", request.controller_name,);

    let mut handshake_stream =
        retry(10, target).ok_or_else(|| anyhow!("Couldn't connect for handshake {}", target))?;

    let request_string = serde_json::to_string(&request)?;

    handshake_stream.write_all(&request_string.as_bytes())?;

    let mut response_string = String::new();
    handshake_stream.read_to_string(&mut response_string)?;
    let response_string = response_string.trim_end_matches('\u{0}');

    let response: Response = serde_json::from_str(&response_string)?;

    if response.status != 0 {
        anyhow!("connection refused - {:?}", response);
    }
    Ok(response)
}

fn retry(times: usize, target: SocketAddr) -> Option<TcpStream> {
    let timeout = std::time::Duration::from_secs(2);
    for retry_time in 0..times {
        match TcpStream::connect_timeout(&target, timeout) {
            Ok(stream) => return Some(stream),
            Err(err) => eprintln!("Error connecting to Tcp ({}): {}", retry_time, err),
        };
    }

    None
}
