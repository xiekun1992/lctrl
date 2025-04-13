use rusqlite::Connection;

use crate::{global::device::RemoteDevice, input::listener::ControlSide};

use super::state::{Rect, RECT};

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
                screen_size_left integer, 
                screen_size_right integer, 
                screen_size_top integer, 
                screen_size_bottom integer, 
                side integer,
                netmask varchar(255)
            )",
            (),
        )
        .unwrap();

        conn.execute(
            "create table if not exists current_device (
                id integer primary key,
                screen_size_left integer, 
                screen_size_right integer, 
                screen_size_top integer, 
                screen_size_bottom integer
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
                r#"
                    insert into remote_peer(
                        hostname, 
                        ip, 
                        screen_size_left, 
                        screen_size_right, 
                        screen_size_top, 
                        screen_size_bottom, 
                        mac_addr, side, netmask
                    ) values (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)"#,
                (
                    &remote_peer.hostname,
                    &remote_peer.ip,
                    &remote_peer.screen_size.left,
                    &remote_peer.screen_size.right,
                    &remote_peer.screen_size.top,
                    &remote_peer.screen_size.bottom,
                    &remote_peer.mac_addr,
                    &remote_peer_side,
                    &remote_peer.netmask,
                ),
            )
            .unwrap();
    }

    pub fn get_remote_peer(&self) -> (Option<RemoteDevice>, ControlSide) {
        match self.conn.prepare(
            r#"select 
                    hostname, ip, 
                    screen_size_left, 
                    screen_size_right, 
                    screen_size_top, 
                    screen_size_bottom, 
                    side, mac_addr, netmask 
                from remote_peer"#,
        ) {
            Ok(mut stmt) => {
                let mut iter = stmt
                    .query_map([], |row| {
                        // println!("{:?}", row);
                        let remote_peer = RemoteDevice {
                            hostname: row.get(0).unwrap_or("".to_string()),
                            ip: row.get(1).unwrap_or("".to_string()),
                            mac_addr: row.get(7).unwrap_or("".to_string()),
                            screen_size: Rect::from(
                                row.get(2).unwrap_or(0),
                                row.get(4).unwrap_or(0),
                                row.get(3).unwrap_or(800),
                                row.get(5).unwrap_or(600),
                            ),
                            screens: vec![Rect {
                                top: 0,
                                right: 1366,
                                bottom: 768,
                                left: 0,
                            }],
                            alive_timestamp: 0,
                            netmask: row.get(8).unwrap_or("".to_string()),
                        };
                        let mut side = ControlSide::NONE;
                        match row.get(6).unwrap() {
                            0 => side = ControlSide::NONE,
                            1 => side = ControlSide::LEFT,
                            2 => side = ControlSide::RIGHT,
                            _ => {}
                        }

                        Ok((remote_peer, side))
                    })
                    .unwrap();
                match iter.next() {
                    Some(res) => match res {
                        Ok((r, s)) => (Some(r), s),
                        Err(e) => {
                            println!("statment iter {:?}", e);
                            (None, ControlSide::NONE)
                        }
                    },
                    _ => (None, ControlSide::NONE),
                }
            }
            Err(e) => {
                println!("select {:?}", e);
                (None, ControlSide::NONE)
            }
        }
    }

    pub fn delete_remote_peer(&self) {
        self.conn.execute("delete from remote_peer", ()).unwrap();
    }

    pub fn set_current_device(&self, screen_size: &RECT) {
        if screen_size.left != screen_size.right && screen_size.top != screen_size.bottom {
            self.delete_current_device();
            self.conn
                .execute(
                    r#"
                        insert into current_device(
                            screen_size_left, 
                            screen_size_right, 
                            screen_size_top, 
                            screen_size_bottom
                        ) values (?1, ?2, ?3, ?4)"#,
                    (
                        &screen_size.left,
                        &screen_size.right,
                        &screen_size.top,
                        &screen_size.bottom,
                    ),
                )
                .unwrap();
        }
    }
    pub fn get_current_device(&self) -> Option<RECT> {
        match self.conn.prepare(
            r#"select 
                    screen_size_left, 
                    screen_size_right, 
                    screen_size_top, 
                    screen_size_bottom
                from current_device"#,
        ) {
            Ok(mut stmt) => {
                let mut iter = stmt
                    .query_map([], |row| {
                        // println!("{:?}", row);
                        Ok(RECT {
                            left: row.get(0).unwrap(),
                            right: row.get(1).unwrap(),
                            top: row.get(2).unwrap(),
                            bottom: row.get(3).unwrap(),
                        })
                    })
                    .unwrap();
                match iter.next() {
                    Some(res) => match res {
                        Ok(r) => Some(r),
                        Err(_e) => None,
                    },
                    _ => None,
                }
            }
            Err(_e) => None,
        }
    }
    pub fn delete_current_device(&self) {
        self.conn.execute("delete from current_device", ()).unwrap();
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
