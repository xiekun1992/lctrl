use std::net::UdpSocket;

use actix_web::{post, web, HttpResponse, Responder};

use crate::{global::state::STATE, web_api::remote_peer::RemoteSetting};

#[post("/launch")]
pub async fn put(setting: web::Query<RemoteSetting>) -> impl Responder {
    let state = STATE.lock().unwrap();
    let remote = state.find_remote_by_ip(&setting.ip.as_str());
    if let Some(rdev) = remote.clone() {
        let socket = UdpSocket::bind("0.0.0.0:0").unwrap();
        let packet = gen_wol_magic_packet(rdev.mac_addr.as_str());
        let addr = format!("{}:{}", rdev.ip, 7);
        socket
            .send_to(packet.as_slice(), addr)
            .expect("WOL magic packet send fail");
    }
    HttpResponse::Ok().json(())
}

fn gen_wol_magic_packet(mac_addr: &str) -> Vec<u8> {
    let mut packet: Vec<u8> = Vec::from([0xff, 0xff, 0xff, 0xff, 0xff, 0xff]);
    let mac = mac_addr
        .split(":")
        .map(|x| u8::from_str_radix(x, 16).unwrap())
        .collect::<Vec<u8>>();
    for _ in 0..16 {
        packet.append(&mut mac.clone());
    }
    packet
}

#[test]
fn test_gen_wol_magic_packet() {
    let packet = gen_wol_magic_packet("0a:00:27:00:00:07");
    // println!("{:?}", packet);
    assert_eq!(
        [
            255, 255, 255, 255, 255, 255, 10, 0, 39, 0, 0, 7, 10, 0, 39, 0, 0, 7, 10, 0, 39, 0, 0,
            7, 10, 0, 39, 0, 0, 7, 10, 0, 39, 0, 0, 7, 10, 0, 39, 0, 0, 7, 10, 0, 39, 0, 0, 7, 10,
            0, 39, 0, 0, 7, 10, 0, 39, 0, 0, 7, 10, 0, 39, 0, 0, 7, 10, 0, 39, 0, 0, 7, 10, 0, 39,
            0, 0, 7, 10, 0, 39, 0, 0, 7, 10, 0, 39, 0, 0, 7, 10, 0, 39, 0, 0, 7, 10, 0, 39, 0, 0,
            7
        ],
        packet.as_slice()
    )
}
