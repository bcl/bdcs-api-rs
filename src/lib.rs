//! BDCS Crate
//!
// Copyright (C) 2016-2017
// Red Hat, Inc.  All rights reserved.
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation; either version 2 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.
//
//! ## Overview
//!
//! The bdcs crate is the library used by the bdcs-api-server.
//!
#![feature(plugin)]
#![feature(proc_macro)]
#![feature(custom_derive)]
#![plugin(rocket_codegen)]

extern crate glob;
extern crate hyper;
#[macro_use] extern crate lazy_static;
extern crate r2d2;
extern crate r2d2_sqlite;
extern crate rocket;
#[macro_use] extern crate rocket_contrib;
extern crate rusqlite;
extern crate rustc_serialize;
extern crate serde_json;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate slog;
#[macro_use] extern crate slog_scope;
extern crate toml;


pub mod api;
pub mod db;
pub mod recipe;
