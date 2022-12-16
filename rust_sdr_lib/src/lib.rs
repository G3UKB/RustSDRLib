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

use std::thread;
use std::time::Duration;
use std::{cell::RefCell, rc::Rc};
use std::io::{stdin, stdout, Read, Write};

pub mod app;

/// Entry point for RustConsole SDR application
///
/// # Examples
///
/// 

struct InitData {
    i_app: app::Appdata,
}

impl InitData {

    #[no_mangle]
    pub extern "C" fn new() -> InitData {
        // Create an instance of the Application manager type
        let mut app = app::Appdata::new();

        InitData {
            i_app: app,
        }
    }

    #[no_mangle]
    pub extern "C" fn sdrlib_run(&mut self) {
        // Start library
        println!("Starting Rust SDR Library...");

        // Create an instance of the Application manager type
        //let mut app = app::Appdata::new();

        // This will initialise all modules and run the back-end system
        self.i_app.app_init();
        println!("Rust SDR Library initilaised\n");
    }

    #[no_mangle]
    pub extern "C" fn sdrlib_close(&mut self) {
        // Close library
        println!("\n\nRust SDR Library shutdown...");
        self.i_app.app_close();

        println!("Rust SDR Library closing...");
        thread::sleep(Duration::from_millis(1000));
    }
}


