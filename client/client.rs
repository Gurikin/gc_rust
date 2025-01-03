// Copyright (C) 2018-2019, Cloudflare, Inc.
// All rights reserved.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions are
// met:
//
//     * Redistributions of source code must retain the above copyright notice,
//       this list of conditions and the following disclaimer.
//
//     * Redistributions in binary form must reproduce the above copyright
//       notice, this list of conditions and the following disclaimer in the
//       documentation and/or other materials provided with the distribution.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS
// IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO,
// THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
// PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR
// CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
// EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
// PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
// PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF
// LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING
// NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
// SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

extern crate pretty_env_logger;
#[macro_use]
extern crate log;

use mio::{net::UdpSocket, Poll};
use quiche::Connection;
use ring::rand::*;
use url::Url;

const MAX_DATAGRAM_SIZE: usize = 1350;

const HTTP_REQ_STREAM_ID: u64 = 4;

pub struct QuicConnection {
    url: Url,
    conn: Connection,
    socket: UdpSocket,
    poll: Poll,
}

impl QuicConnection {
    pub fn connect(url: String) -> Self {
        let url = Url::parse(&url).unwrap();

        // Resolve server address.
        let peer_addr = url.socket_addrs(|| None).unwrap()[0];

        // Bind to INADDR_ANY or IN6ADDR_ANY depending on the IP family of the
        // server address. This is needed on macOS and BSD variants that don't
        // support binding to IN6ADDR_ANY for both v4 and v6.
        let bind_addr = match peer_addr {
            std::net::SocketAddr::V4(_) => "0.0.0.0:0",
            std::net::SocketAddr::V6(_) => "[::]:0",
        };

        // Setup the event loop.
        let poll = Poll::new().unwrap();

        // Create the UDP socket backing the QUIC connection, and register it with
        // the event loop.
        let mut socket = mio::net::UdpSocket::bind(bind_addr.parse().unwrap()).unwrap();
        poll.registry()
            .register(&mut socket, mio::Token(0), mio::Interest::READABLE)
            .unwrap();

        // Create the configuration for the QUIC connection.
        let mut config = quiche::Config::new(quiche::PROTOCOL_VERSION).unwrap();

        // *CAUTION*: this should not be set to `false` in production!!!
        config.verify_peer(false);

        config
            .set_application_protos(&[b"hq-interop", b"hq-29", b"hq-28", b"hq-27", b"http/0.9", b"http/1.1"])
            .unwrap();

        match config.load_verify_locations_from_directory("./") {
            Ok(_) => (),
            Err(e) => error!("Load verify location error. {}", e),
        };
        match config.load_cert_chain_from_pem_file("cert.crt") {
            Ok(_) => (),
            Err(e) => error!("Load cert error. {}", e),
        }

        match config.load_priv_key_from_pem_file("cert.key") {
            Ok(_) => (),
            Err(e) => error!("Load private key error. {}", e),
        }

        config.set_max_idle_timeout(5000);
        config.set_max_recv_udp_payload_size(MAX_DATAGRAM_SIZE);
        config.set_max_send_udp_payload_size(MAX_DATAGRAM_SIZE);
        config.set_initial_max_data(10_000_000);
        config.set_initial_max_stream_data_bidi_local(1_000_000);
        config.set_initial_max_stream_data_bidi_remote(1_000_000);
        config.set_initial_max_streams_bidi(100);
        config.set_initial_max_streams_uni(100);
        config.set_disable_active_migration(true);

        // Generate a random source connection ID for the connection.
        let mut scid = [0; quiche::MAX_CONN_ID_LEN];
        SystemRandom::new().fill(&mut scid[..]).unwrap();

        let scid = quiche::ConnectionId::from_ref(&scid);

        // Get local address.
        let local_addr = socket.local_addr().unwrap();

        // Create a QUIC connection and initiate handshake.
        let conn =
            quiche::connect(url.domain(), &scid, local_addr, peer_addr, &mut config).unwrap();

        info!(
            "connecting to {:} from {:} with scid {}",
            peer_addr,
            socket.local_addr().unwrap(),
            hex_dump(&scid)
        );
        Self {
            url,
            conn,
            socket,
            poll,
        }
    }
}

fn main() {
    pretty_env_logger::init_timed();
    let mut args = std::env::args();

    let cmd = &args.next().unwrap();

    if args.len() < 1 {
        println!("Usage: {cmd} URL BODY");
        println!("\nSee tools/apps/ for more complete implementations.");
        return;
    }
    let mut quic_connection = QuicConnection::connect(args.next().unwrap());

    // let mut out = [0; MAX_DATAGRAM_SIZE];
    let body = args.next().unwrap_or("Hello quic".to_string());
    let mut out = <[u8; 1350]>::try_from(body.as_bytes()).unwrap_or([0 as u8; 1350]);
    let (write, send_info) = quic_connection
        .conn
        .send(&mut out)
        .expect("initial send failed");

    while let Err(e) = quic_connection.socket.send_to(&out[..write], send_info.to) {
        if e.kind() == std::io::ErrorKind::WouldBlock {
            debug!("send() would block");
            continue;
        }

        panic!("send() failed: {:?}", e);
    }

    debug!("written {}", write);

    let req_start = std::time::Instant::now();

    let mut req_sent = false;

    let mut buf = [0; 65535];
    let mut events = mio::Events::with_capacity(1024);
    loop {
        quic_connection
            .poll
            .poll(&mut events, quic_connection.conn.timeout())
            .unwrap();

        // Read incoming UDP packets from the socket and feed them to quiche,
        // until there are no more packets to read.
        'read: loop {
            // If the event loop reported no events, it means that the timeout
            // has expired, so handle it without attempting to read packets. We
            // will then proceed with the send loop.
            if events.is_empty() {
                debug!("timed out");

                quic_connection.conn.on_timeout();
                break 'read;
            }

            let (len, from) = match quic_connection.socket.recv_from(&mut buf) {
                Ok(v) => v,

                Err(e) => {
                    // There are no more UDP packets to read, so end the read
                    // loop.
                    if e.kind() == std::io::ErrorKind::WouldBlock {
                        debug!("recv() would block");
                        break 'read;
                    }

                    panic!("recv() failed: {:?}", e);
                }
            };

            debug!("got {} bytes", len);

            let recv_info = quiche::RecvInfo {
                to: quic_connection.socket.local_addr().unwrap(),
                from,
            };

            // Process potentially coalesced packets.
            let read = match quic_connection.conn.recv(&mut buf[..len], recv_info) {
                Ok(v) => v,

                Err(e) => {
                    error!("recv failed: {:?}", e);
                    continue 'read;
                }
            };

            debug!("processed {} bytes", read);
        }

        debug!("done reading");

        if quic_connection.conn.is_closed() {
            info!("connection closed, {:?}", quic_connection.conn.stats());
            break;
        }

        // Send an HTTP request as soon as the connection is established.
        if quic_connection.conn.is_established() && !req_sent {
            info!("sending HTTP request for {}", quic_connection.url.path());

            let req = format!("GET {}\r\n", quic_connection.url.path());
            quic_connection
                .conn
                .stream_send(HTTP_REQ_STREAM_ID, req.as_bytes(), true)
                .unwrap();

            req_sent = true;
        }

        // Process all readable streams.
        for s in quic_connection.conn.readable() {
            while let Ok((read, fin)) = quic_connection.conn.stream_recv(s, &mut buf) {
                debug!("received {} bytes", read);

                let stream_buf = &buf[..read];

                debug!("stream {} has {} bytes (fin? {})", s, stream_buf.len(), fin);

                print!("{}", unsafe { std::str::from_utf8_unchecked(stream_buf) });

                // The server reported that it has no more data to send, which
                // we got the full response. Close the connection.
                if s == HTTP_REQ_STREAM_ID && fin {
                    info!("response received in {:?}, closing...", req_start.elapsed());

                    quic_connection.conn.close(true, 0x00, b"kthxbye").unwrap();
                }
            }
        }

        // Generate outgoing QUIC packets and send them on the UDP socket, until
        // quiche reports that there are no more packets to be sent.
        loop {
            let (write, send_info) = match quic_connection.conn.send(&mut out) {
                Ok(v) => v,

                Err(quiche::Error::Done) => {
                    debug!("done writing");
                    debug!("{}", std::str::from_utf8(&out).unwrap_or("Bad Request"));
                    break;
                }

                Err(e) => {
                    error!("send failed: {:?}", e);

                    quic_connection.conn.close(false, 0x1, b"fail").ok();
                    break;
                }
            };

            if let Err(e) = quic_connection.socket.send_to(&out[..write], send_info.to) {
                if e.kind() == std::io::ErrorKind::WouldBlock {
                    debug!("send() would block");
                    break;
                }

                panic!("send() failed: {:?}", e);
            }

            debug!("written {}", write);
        }

        if quic_connection.conn.is_closed() {
            info!("connection closed, {:?}", quic_connection.conn.stats());
            break;
        }
    }
}

fn hex_dump(buf: &[u8]) -> String {
    let vec: Vec<String> = buf.iter().map(|b| format!("{b:02x}")).collect();

    vec.join("")
}
