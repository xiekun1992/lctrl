use std::{
    net::{Ipv4Addr, UdpSocket},
    str::FromStr,
};

use actix_web::{post, web, HttpResponse, Responder};
use tracing::{debug, info};

use crate::{
    global::{device::calc_broadcast_addr, state::STATE},
    web_api::remote_peer::RemoteSetting,
};

#[post("/launch")]
pub async fn put(setting: web::Query<RemoteSetting>) -> impl Responder {
    let state = STATE.lock().unwrap();
    if let Some(remote) = state.get_remote_peer() {
        if setting.ip.eq(&remote.ip) && state.side.eq(&setting.side) {
            debug!("{:?}", remote);
            let socket = UdpSocket::bind("0.0.0.0:18001").unwrap();
            let packet = gen_wol_magic_packet(remote.mac_addr.as_str());
            let ip_addr = Ipv4Addr::from_str(remote.ip.as_str()).unwrap();
            let netmask = Ipv4Addr::from_str(remote.netmask.as_str()).unwrap();

            let broadcast_addr = calc_broadcast_addr(ip_addr, netmask);
            let addr = format!("{}:{}", broadcast_addr.clone().to_string(), 7);
            info!(
                "magic packet sent: {:?}, ip: {:?}, netmask: {:?}",
                addr,
                ip_addr.clone(),
                netmask.clone()
            );
            socket
                .send_to(packet.as_slice(), addr)
                .expect("WOL magic packet send fail");

            HttpResponse::Ok().json(())
        } else {
            HttpResponse::NotFound().json(())
        }
    } else {
        HttpResponse::NotFound().json(())
    }
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
