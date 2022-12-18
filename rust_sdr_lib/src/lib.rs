/*
lib.rs

Entry module for the RustConsoleLib library SDR application

Copyright (C) 2022 by G3UKB Bob Cowdery

This program is free software; you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation; either version 2 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program; if not, write to the Free Software
Foundation, Inc., 59 Temple Place, Suite 330, Boston, MA  02111-1307  USA

The authors can be reached by email at:

bob@bobcowdery.plus.com
*/

use std::thread::{self, JoinHandle};
use std::time::Duration;
use crossbeam_channel::unbounded;
use std::sync::Mutex;
use std::ffi::CString;

extern crate serde;

use crate::app::common::common_defs;
use crate::app::common::messages;
use crate::app::dsp;

pub mod app;

//#[derive(Debug)]
pub struct InitData {
    // Channel
    pub sender : crossbeam_channel::Sender<messages::AppMsg>,
    pub receiver : crossbeam_channel::Receiver<messages::AppMsg>,
    pub handle: Option<JoinHandle<()>>,
}

static GLOBAL_DATA: Mutex<Option<InitData>> = Mutex::new(None);

#[no_mangle]
pub fn sdrlib_init() {
    let mut guard = GLOBAL_DATA.lock().unwrap();
    let init_data = &mut *guard;
    let (s, r) = unbounded();
    init_data.replace(InitData { 
        sender: s, 
        receiver: r, 
        handle: None,
    });
}

#[no_mangle]
pub extern "C" fn sdrlib_run() {
    let mut guard = GLOBAL_DATA.lock().unwrap();
    let init_data = guard.as_mut().expect("Call init before using sdrlib_run");
    // Start library
    let handle = app::app_start(init_data.receiver.clone());
    init_data.handle = Some(handle);
    println!("Started Rust SDR Server");
}

#[no_mangle]
pub extern "C" fn sdrlib_close() {
    let mut guard = GLOBAL_DATA.lock().unwrap();
    let init_data = guard.as_mut().expect("Call init before using sdrlib_run");
    println!("\n\nRust SDR Server shutdown...");
    let msg = messages::AppMsg {msg_type: messages::AppMsgType::Terminate, param1: 0};
    init_data.sender
        .clone()
        .send(msg)
        .expect("Channel should not be closed yet");
    if let Some(h) = init_data.handle.take() {
        println!("Waiting for application thread to terminate...");
        h.join().expect("Failed to join application thread!");
        println!("Application thread terminated");
    }

    println!("Rust SDR Server closing...");
    thread::sleep(Duration::from_millis(1000));
}

#[no_mangle]
pub extern "C" fn sdrlib_freq(freq: u32) {
    let mut guard = GLOBAL_DATA.lock().unwrap();
    let init_data = guard.as_mut().expect("Call init before using sdrlib_run");
    let msg = messages::AppMsg {msg_type: messages::AppMsgType::Frequency, param1: freq};
    init_data.sender
        .clone()
        .send(msg)
        .expect("Channel rejected frequency command");
}

#[no_mangle]
pub extern "C" fn sdrlib_mode(mode: i32) {
    dsp::dsp_interface::wdsp_set_rx_mode(0, mode);
}

#[no_mangle]
pub extern "C" fn sdrlib_filter(filter: i32) {
    dsp::dsp_interface::wdsp_set_rx_filter(0, filter);
}

#[no_mangle]
pub extern "C" fn sdrlib_disp_data() -> CString {
    let mut out_real = [0.0; (common_defs::DSP_BLK_SZ ) as usize];
    dsp::dsp_interface::wdsp_get_display_data(0, &mut out_real);
    let c_str = CString::new(serde_json::to_string(&out_real.to_vec()).unwrap());
    println!("{:?}", c_str);
    match c_str {
        Ok(s) => return s,
        Err(_) => return CString::new("").expect("CString new failed"),
    }
}


