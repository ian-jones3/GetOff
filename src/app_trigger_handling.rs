use procfs::process::{Stat, all_processes};
use std::process::Command;

use crate::app::App;

struct ProcessToHandle {
    stat: Stat, // process status info
}

pub fn check_running_processes(app: &App) {
    let full_process_list = match all_processes() {
        Ok(full_process_list) => full_process_list,
        Err(err) => return,
    };

    // for proc in full_process_list {
    //     // comm is filename of executable, we can match this with
    //     // items on application list.
    //     // We most likely won't need any more information beyond
    //     // what is contained in Stat
    //     //println!("{:?}", &proc.unwrap().stat().unwrap().comm);
    //
    //     // just an example implementation of killing a process,
    //     // only targeting spotify here
    //     let proc = &proc.unwrap().stat().unwrap();
    //     if proc.comm == "spotify" {
    //         println!("KILLING SPOTIFY");
    //         Command::new("kill")
    //             .arg("-9")
    //             .arg(proc.pid.to_string())
    //             .status();
    //     }
    // }
    // for a in &app.confirmed_app_list.applications {
    //     println!("About to print app name!");
    //     println!("{}", &a.name);
    // }
}
