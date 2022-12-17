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
use std::{cell::RefCell, rc::Rc};
use std::io::{stdin, stdout, Read, Write};
use std::option;
use crossbeam_channel::unbounded;
use std::ops::{Deref, DerefMut};
use std::sync::{Arc, Mutex, Condvar};
#[macro_use]
extern crate lazy_static;

pub mod app;
#[derive(Debug)]
pub struct InitData {
    // Channel
    pub sender : crossbeam_channel::Sender<i32>,
    pub receiver : crossbeam_channel::Receiver<i32>,
    pub handle: Mutex<option::Option<JoinHandle<()>>>,
}

impl InitData {
    pub fn new() -> InitData {
        let (s, r) = unbounded();
        InitData {
            sender: s,
            receiver: r,
            handle: Mutex::new(None),
        }
    }
    pub fn set_handle(&mut self, h:JoinHandle<()>) {
        *self.handle.lock().unwrap() = Some(h);
    }

     
    pub fn wait_handle(&mut self) {
        
        let mut x = self.handle.lock().unwrap().take();
        //if let Some(y) = x.take() {
            //x.join().unwrap().expect("Join Pipeline failed!");
            x.take().unwrap().join();
        //}
    }
    
/* 
    pub fn wait_handle(&mut self) {
        match &self.handle {
            None => (),
            Some(h) => {
                match h.lock() {
                    Ok(h) => h.join(),
                    Err(e) => e.into_inner(),
                }
            }
        }
    }
*/
    
}

lazy_static! {
    pub static ref init: InitData = InitData::new();
}

#[no_mangle]
pub extern "C" fn sdrlib_run() {

    // Start library
    let handle = app::app_start(init.receiver.clone());
    init.set_handle(handle);
    println!("Started Rust SDR Server");
}

#[no_mangle]
pub extern "C" fn sdrlib_close() {
    // Close library
    println!("\n\nRust SDR Server shutdown...");
    init.sender.clone().send(0);
    init.wait_handle();
    println!("Rust SDR Server closing...");
    thread::sleep(Duration::from_millis(1000));
}


