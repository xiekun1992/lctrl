use rusqlite::Connection;

use crate::{
    global::{
        device::RemoteDevice,
        setting::{self, Setting},
    },
    input::listener::ControlSide,
    web_api::dto::ScreenSetting,
};

use super::state::{Rect, RECT};
use std::{any::Any, vec::Vec};
use tracing::{error, info};

pub struct DB {
    conn: Connection,
}

impl DB {
    pub fn new() -> Self {
        if let Ok(conn) = Connection::open("lctrl.db") {
            match conn.execute_batch(
                "
                BEGIN;
                    create table if not exists remote_peer (
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
                    );
                    create table if not exists current_device (
                        id integer primary key,
                        screen_size_left integer, 
                        screen_size_right integer, 
                        screen_size_top integer, 
                        screen_size_bottom integer
                    );
                    create table if not exists screens (
                        id integer primary key,
                        screen_size_left integer, 
                        screen_size_right integer, 
                        screen_size_top integer, 
                        screen_size_bottom integer
                    );
                    create table if not exists setting (
                        id integer primary key,
                        auto_discover integer,
                        scale_factor decimal,
                        cursor_across_screens integer,
                        mouse_wheel_style integer,
                        enable_control integer
                    );
                COMMIT;
                ",
            ) {
                Ok(_) => {}
                Err(e) => {
                    error!("create table error {:?}", e);
                }
            }

            DB { conn }
        } else {
            panic!("open database lctrl.db failed");
        }
    }

    pub fn initialize(&self) {
        self.init_setting();
    }

    pub fn init_setting(&self) {
        let setting = self.get_setting();
        self.set_setting(&setting);
        info!("initialize setting {:?}", setting);
    }

    pub fn set_setting(&self, setting: &Setting) {
        self.conn
            .execute("delete from setting", ())
            .unwrap_or_default();
        match self.conn.execute(
            "
                    insert into setting (
                        auto_discover integer,
                        scale_factor decimal,
                        cursor_across_screens integer,
                        mouse_wheel_style integer,
                        enable_control integer
                    ) values (?1, ?2, ?3, ?4, ?5)
                ",
            (
                &setting.auto_discover,
                &setting.scale_factor,
                &setting.cursor_across_screens,
                setting.mouse_wheel_style.clone() as i32,
                &setting.enable_control,
            ),
        ) {
            Ok(s) => {
                println!("insert setting affected rows {}", s);
            }
            Err(e) => {
                println!("insert setting error {:?}", e);
            }
        }
    }

    pub fn get_setting(&self) -> Setting {
        match self.conn.prepare(
            r#"select 
                    auto_discover,
                    scale_factor,
                    cursor_across_screens,
                    mouse_wheel_style,
                    enable_control
                from setting"#,
        ) {
            Ok(mut stmt) => {
                if let Ok(mut iter) = stmt.query_map([], |row| {
                    // println!("{:?}", row);
                    Ok(Setting {
                        auto_discover: row.get(0).unwrap_or(true),
                        scale_factor: row.get(1).unwrap_or(1.0),
                        cursor_across_screens: row.get(2).unwrap_or(true),
                        mouse_wheel_style: match row.get::<_, i32>(3).unwrap_or(0) {
                            0 => setting::MouseWheelStyle::Traditional,
                            1 => setting::MouseWheelStyle::Natural,
                            _ => setting::MouseWheelStyle::Traditional,
                        },
                        screen_setting: ScreenSetting::new(),
                        enable_control: row.get(4).unwrap_or(true),
                    })
                }) {
                    match iter.next() {
                        Some(res) => match res {
                            Ok(s) => s,
                            Err(_e) => Setting::default(),
                        },
                        _ => Setting::default(),
                    }
                } else {
                    info!("get_setting query_map failed");
                    Setting::default()
                }
            }
            Err(_e) => {
                error!("get_setting select failed");
                Setting::default()
            }
        }
    }

    pub fn delete_screens(&self) {
        self.conn
            .execute("delete from screens", ())
            .unwrap_or_default();
    }
    pub fn set_screens(&self, screens: &Vec<Rect>) {
        self.delete_screens();
        for screen in screens.iter() {
            match self.conn.execute(
                "
                        insert into screens(
                            screen_size_left, 
                            screen_size_right, 
                            screen_size_top, 
                            screen_size_bottom
                        ) values (?1, ?2, ?3, ?4)
                ",
                (&screen.left, &screen.right, &screen.top, &screen.bottom),
            ) {
                Ok(s) => {
                    println!("insert screens affected rows {}", s);
                }
                Err(e) => {
                    println!("insert screens error {:?}", e);
                }
            }
        }
    }

    pub fn get_screens(&self) -> Vec<Rect> {
        match self.conn.prepare(
            r#"select 
                    screen_size_left, 
                    screen_size_right, 
                    screen_size_top, 
                    screen_size_bottom
                from screens"#,
        ) {
            Ok(mut stmt) => {
                if let Ok(iter) = stmt.query_map([], |row| {
                    // println!("{:?}", row);
                    Ok(Rect {
                        left: row.get(0).unwrap_or(0),
                        right: row.get(1).unwrap_or(800),
                        top: row.get(2).unwrap_or(0),
                        bottom: row.get(3).unwrap_or(600),
                    })
                }) {
                    let mut screens = vec![];
                    for res in iter {
                        match res {
                            Ok(r) => screens.push(r),
                            Err(_e) => {}
                        }
                    }
                    screens
                } else {
                    info!("get_screens query_map failed");
                    vec![]
                }
            }
            Err(_e) => {
                error!("get_screens select failed");
                vec![]
            }
        }
    }

    pub fn set_remote_peer(&self, remote_peer: &RemoteDevice, side: &ControlSide) {
        self.delete_remote_peer();
        let remote_peer_side = match side {
            ControlSide::NONE => 0,
            ControlSide::LEFT => 1,
            ControlSide::RIGHT => 2,
            ControlSide::TOP => 3,
        };
        match self.conn.execute(
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
        ) {
            Ok(s) => {
                println!("insert remote peer affected rows {}", s);
            }
            Err(e) => {
                println!("insert remote peer error {:?}", e);
            }
        }
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
                if let Ok(mut iter) = stmt.query_map([], |row| {
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
                    match row.get(6).unwrap_or(0) {
                        0 => side = ControlSide::NONE,
                        1 => side = ControlSide::LEFT,
                        2 => side = ControlSide::RIGHT,
                        _ => {}
                    }

                    Ok((remote_peer, side))
                }) {
                    match iter.next() {
                        Some(res) => match res {
                            Ok((r, s)) => (Some(r), s),
                            Err(e) => {
                                info!("get_remote_peer statment iter {:?}", e);
                                (None, ControlSide::NONE)
                            }
                        },
                        _ => (None, ControlSide::NONE),
                    }
                } else {
                    info!("get_remote_peer query_map failed");
                    (None, ControlSide::NONE)
                }
            }
            Err(e) => {
                error!("get_remote_peer select {:?}", e);
                (None, ControlSide::NONE)
            }
        }
    }
    pub fn delete_remote_peer(&self) {
        match self.conn.execute("delete from remote_peer", ()) {
            Ok(s) => {
                info!("delete remote peer affected rows {}", s);
            }
            Err(e) => {
                error!("delete remote peer error {:?}", e);
            }
        }
    }

    pub fn set_current_device(&self, screen_size: &RECT) {
        if screen_size.left != screen_size.right && screen_size.top != screen_size.bottom {
            self.delete_current_device();
            match self.conn.execute(
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
            ) {
                Ok(r) => {
                    info!("insert current device affected rows {}", r);
                }
                Err(e) => {
                    error!("insert current device error {:?}", e);
                }
            }
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
                if let Ok(mut iter) = stmt.query_map([], |row| {
                    // println!("{:?}", row);
                    Ok(RECT {
                        left: row.get(0).unwrap_or(0),
                        right: row.get(1).unwrap_or(800),
                        top: row.get(2).unwrap_or(0),
                        bottom: row.get(3).unwrap_or(600),
                    })
                }) {
                    match iter.next() {
                        Some(res) => match res {
                            Ok(r) => Some(r),
                            Err(_e) => None,
                        },
                        _ => None,
                    }
                } else {
                    info!("get_current_device query_map failed");
                    None
                }
            }
            Err(_e) => {
                error!("get_current_device select failed");
                None
            }
        }
    }
    pub fn delete_current_device(&self) {
        match self.conn.execute("delete from current_device", ()) {
            Ok(r) => {
                info!("delete current device affected rows {}", r);
            }
            Err(e) => {
                error!("delete current device error {:?}", e);
            }
        }
    }
}
