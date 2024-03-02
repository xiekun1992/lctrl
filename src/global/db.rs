use std::sync::Mutex;

use rusqlite::Connection;

use crate::{
    global::device::RemoteDevice,
    input::listener::{ControlSide, SIDE},
};
use lazy_static::lazy_static;

lazy_static! {
    pub static ref DB_CONN: Mutex<DB> = Mutex::new(DB::new());
}

pub struct DB {
    conn: Connection,
}

impl DB {
    pub fn new() -> Self {
        let conn = Connection::open("lctrl.db").unwrap();
        conn.execute(
            "create table if not exists remote_peer (
                id integer primary key,
                hostname varchar(255),
                ip varchar(255),
                mac_addr varchar(255),
                screen_size_x integer,
                screen_size_y integer,
                side integer
            )",
            (),
        )
        .unwrap();
        DB { conn }
    }
    pub fn set_remote_peer(&self, remote_peer: &RemoteDevice, side: &ControlSide) {
        self.delete_remote_peer();
        let remote_peer_side = match side {
            ControlSide::NONE => 0,
            ControlSide::LEFT => 1,
            ControlSide::RIGHT => 2,
        };
        self.conn
            .execute(
                "insert into remote_peer(hostname, ip, screen_size_x, screen_size_y, mac_addr, side) values (?1, ?2, ?3, ?4, ?5, ?6)",
                (&remote_peer.hostname, &remote_peer.ip, &remote_peer.screen_size[0], &remote_peer.screen_size[1], &remote_peer.mac_addr, &remote_peer_side),
            )
            .unwrap();
    }
    pub fn get_remote_peer(&self) -> Option<RemoteDevice> {
        match self.conn.prepare(
            "select hostname, ip, screen_size_x, screen_size_y, side, mac_addr from remote_peer",
        ) {
            Ok(mut stmt) => {
                let mut iter = stmt
                    .query_map([], |row| {
                        println!("{:?}", row);
                        let remote_peer = RemoteDevice {
                            hostname: row.get(0).unwrap(),
                            ip: row.get(1).unwrap(),
                            mac_addr: row.get(5).unwrap(),
                            screen_size: [row.get(2).unwrap(), row.get(3).unwrap()],
                        };
                        unsafe {
                            match row.get(4).unwrap() {
                                0 => SIDE = ControlSide::NONE,
                                1 => SIDE = ControlSide::LEFT,
                                2 => SIDE = ControlSide::RIGHT,
                                _ => {}
                            }
                        }

                        Ok(remote_peer)
                    })
                    .unwrap();
                match iter.next() {
                    Some(res) => match res {
                        Ok(r) => Some(r),
                        Err(e) => {
                            println!("statment iter {:?}", e);
                            None
                        }
                    },
                    _ => None,
                }
            }
            Err(e) => {
                println!("select {:?}", e);
                None
            }
        }
    }
    pub fn delete_remote_peer(&self) {
        self.conn.execute("delete from remote_peer", ()).unwrap();
    }
}

// let me = Person {
//     id: 0,
//     name: "xk".to_string(),
//     data: None,
// };
// conn.execute(
//     "insert into person(name, data) values (?1, ?2)",
//     (&me.name, &me.data),
// )
// .unwrap();
// let mut stmt = conn.prepare("select id, name, data from person").unwrap();
// let person_iter = stmt
//     .query_map([], |row| {
//         Ok(Person {
//             id: row.get(0).unwrap(),
//             name: row.get(1).unwrap(),
//             data: row.get(2).unwrap(),
//         })
//     })
//     .unwrap();

// for person in person_iter {
//     println!("Found person {:?}", person.unwrap());
// }
