extern crate bdcs;
extern crate r2d2;
extern crate r2d2_sqlite;
extern crate rusqlite;
#[macro_use] extern crate slog;
#[macro_use] extern crate slog_scope;
extern crate slog_term;

use bdcs::db::*;
use bdcs::depclose::*;
use bdcs::depsolve::*;

use r2d2_sqlite::SqliteConnectionManager;
use slog::DrainExt;
use std::env;
use std::process::exit;
use std::rc::Rc;

macro_rules! exit_error {
    ($rc:expr, $msg:expr) => ( { println!("error: {}", $msg); exit($rc); } )
}

fn main() {
    let mut argv: Vec<String> = env::args().collect();
    if argv.len() < 3 { exit_error!(2, "depclose metadata.db RPM [RPM...]"); }

    // Remove the program, grab the database.
    argv.remove(0);
    let db = argv.remove(0);

    let term_drain = slog_term::streamer().build();
    let log = slog::Logger::root(term_drain.fuse(), o!());
    slog_scope::set_global_logger(log);

    info!("depclose"; "mddb" => db, "rpms" => format!("{:?}", argv));

    // connect to the database
    let cfg = r2d2::Config::builder().build();
    let mgr = SqliteConnectionManager::new(db.as_str());
    let pool = r2d2::Pool::new(cfg, mgr)
        .unwrap_or_else(|e| exit_error!(3, e));
    let conn = pool.get()
        .unwrap_or_else(|e| exit_error!(3, e));

    info!("Depclosing...");
    // depclose the given args into a big ol' depexpr
    let depexpr = close_dependencies(&conn, &vec!(String::from("x86_64")), &argv)
        .unwrap_or_else(|e| exit_error!(1, e));
    info!("Depclose done.");

    // Wrap the returned depexpression in the crud it needs
    let mut exprs = vec![Rc::new(DepCell::new(depexpr))];

    info!("Depsolving...");
    match solve_dependencies(&conn, &mut exprs) {
        Ok(ids) => { let mut results = Vec::new();
                     for id in ids {
                         match get_groups_id(&conn, &id) {
                             // Commented out for the moment - just printing group names is easier
                             // to debug.
                             // Ok(Some(grp)) => { let mut details = get_projects_details(&conn, &[grp.name.as_str()], 0, -1).unwrap();
                             //                    results.append(&mut details);
                             //                  }
                             Ok(Some(grp)) => { results.push(grp.name) }
                             _ => { }
                         }
                     }

                     for x in results {
                         println!("{}", x);
                     }
                   }
        Err(e)  => { println!("{}", e); }
    }
}
