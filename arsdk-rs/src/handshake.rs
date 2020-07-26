use log::{error, info};
use serde::{Deserialize, Serialize};
use serde_with::with_prefix;
use std::{
    io::{Read, Write},
    net::{Shutdown, SocketAddr, TcpStream},
    string::FromUtf8Error,
};
use thiserror::Error;

#[derive(Serialize, Debug)]
pub(crate) struct Request {
    controller_name: String,
    controller_type: String,
    d2c_port: u16,
    #[serde(flatten, with = "prefix_arstream2_client")]
    pub arstream2: Option<ArStream2>,
}

#[derive(Deserialize, Debug)]
/// Request: "{\"controller_name\":\"arsdk-rs\",\"controller_type\":\"computer\",\"d2c_port\":43210}"
/// Response: "{ \"status\": 0, \"c2d_port\": 54321, \"c2d_update_port\": 51, \"c2d_user_port\": 21, \"qos_mode\": 0, \"arstream2_server_stream_port\": 5004, \"arstream2_server_control_port\": 5005 }\u{0}"
/// `\u{0}` causes issues, but for now we `trim_end_matches`
/// Error: trailing characters at line 1 column 171
pub struct Response {
    #[serde(default)]
    pub arstream_fragment_maximum_number: Option<u8>,
    #[serde(default)]
    pub arstream_fragment_size: Option<u16>,
    #[serde(default)]
    pub arstream_max_ack_interval: Option<i8>,
    #[serde(default, flatten, with = "prefix_arstream2_server")]
    pub arstream2: Option<ArStream2>,
    pub c2d_port: u16,
    pub c2d_update_port: u16,
    pub c2d_user_port: u16,
    pub status: i8,
    // @TODO: Check what this field is for.
    /// Bool?!
    pub qos_mode: u8,
    // @TODO: Check what this field is for **and** if it's a bool at all
    /// Bool?!
    proto_v: u8,
}

with_prefix!(prefix_arstream2_client "arstream2_client_");
with_prefix!(prefix_arstream2_server "arstream2_server_");

#[derive(Debug, Serialize, Deserialize)]
pub struct ArStream2 {
    stream_port: u16,
    control_port: u16,
}
#[derive(Debug, Error)]
pub enum Error {
    #[error("Couldn't connect for handshake {0:?}")]
    Io(#[from] std::io::Error),
    #[error("Connection refused - {0:?}")]
    ConnectionRefused(Response),
    #[error("Maximum allowed retries reached for {target}")]
    Retry { target: SocketAddr },
    #[error("Json (de)serialization - {0:?}")]
    Json(#[from] serde_json::Error),
    /// Primarily used for logging the response string
    #[error("Response String - {0:?}")]
    ResponseString(#[from] FromUtf8Error),
}

pub(crate) fn perform_handshake(
    init_address: SocketAddr,
    d2c_port: u16,
) -> Result<Response, Error> {
    let request = Request {
        controller_name: "arsdk-rs".to_string(),
        controller_type: "computer".to_string(),
        d2c_port,
        // Anafi4k:
        // self.stream_port = 55004
        // self.stream_control_port = 55005
        arstream2: Some(ArStream2 {
            stream_port: 44445,
            control_port: 44446,
        }),
        // TODO: Check when we don't need `arstream2`
        // the pyparrot has a check for setting `arstream2` when:
        // `if(self.drone_type in ("Anafi", "Bebop", "Bebop2", "Disco")):`
        // arstream2: None,
    };

    info!("Connecting controller {}", request.controller_name);

    let mut handshake_stream = retry(10, init_address)?;

    info!("Request: {}", serde_json::to_string(&request)?);
    let request_string = serde_json::to_vec(&request)?;

    handshake_stream.write_all(&request_string)?;

    let mut buf = [0_u8; 256];
    let read = handshake_stream.read(&mut buf)?;
    info!("Read {} bytes!", read);

    let response_string = String::from_utf8(buf[..read].to_vec())?;

    info!("Response: {}", response_string);

    handshake_stream.shutdown(Shutdown::Both)?;

    let response: Response = serde_json::from_str(&response_string)?;

    if response.status != 0 {
        Err(Error::ConnectionRefused(response))
    } else {
        Ok(response)
    }
}

fn retry(times: usize, target: SocketAddr) -> Result<TcpStream, Error> {
    let connection_timeout = std::time::Duration::from_secs(2);
    let read_timeout = std::time::Duration::from_secs(2);
    let mut retry = 0;

    let mut res = TcpStream::connect_timeout(&target, connection_timeout);

    while res.is_err() && retry < times {
        retry += 1;
        res = TcpStream::connect_timeout(&target, connection_timeout);
    }

    let tcp_stream = match res {
        Ok(tcp_stream) => tcp_stream,
        Err(err) => {
            error!("TCP Stream failed: {}", &err);

            return Err(err.into());
        }
    };

    info!("{}: TCP Stream initialized", target);

    tcp_stream.set_read_timeout(Some(read_timeout))?;

    Ok(tcp_stream)
}
