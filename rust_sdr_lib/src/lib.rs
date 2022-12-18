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

pub mod app;

//#[derive(Debug)]
pub struct InitData {
    // Channel
    pub sender : crossbeam_channel::Sender<i32>,
    pub receiver : crossbeam_channel::Receiver<i32>,
    pub handle: Option<JoinHandle<()>>,
}

impl InitData {
    pub fn new() -> InitData {
        let (s, r) = unbounded();
        InitData {
            sender: s,
            receiver: r,
            handle: None,
        }
    }
    pub fn set_handle(&mut self, h:JoinHandle<()>) {
        self.handle = Some(h);
    }
    
}

// Using the strategy from https://doc.rust-lang.org/nomicon/ffi.html#representing-opaque-structs for creating opaque types
// This allows us to avoid exposing any details about InitData to the C API
#[repr(C)]
pub struct AppContext {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

#[no_mangle]
pub extern "C" fn sdrlib_run() -> *mut AppContext {
    let mut init = Box::new(InitData::new());
    // Start library
    let handle = app::app_start(init.receiver.clone());
    init.set_handle(handle);
    println!("Started Rust SDR Server");

    Box::into_raw(init).cast()
}

#[no_mangle]
pub extern "C" fn sdrlib_close(ctx: *mut AppContext) {
    let mut init = unsafe { Box::from_raw(ctx.cast::<InitData>()) };
    println!("\n\nRust SDR Server shutdown...");
    init.sender
        .clone()
        .send(0)
        .expect("Channel should not be closed yet");
    if let Some(h) = init.handle.take() {
        println!("Waiting for application thread to terminate...");
        h.join().expect("Failed to join application thread!");
        println!("Application thread terminated");
    }

    println!("Rust SDR Server closing...");
    thread::sleep(Duration::from_millis(1000));
}


