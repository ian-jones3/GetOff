use std::env;
use std::fs::{self, DirEntry};
use std::io;
use std::path::*;

use tui_widgets::prompts::{State, TextState};

use crate::app::App;

// Only worrying about linux for now,
// but windows and mac could certainly be in
// the future
pub enum OS {
    Linux,
}

// shouldn't be public in the long term
pub struct ApplicationList {
    // likely will be more elaborate than just
    // this vec, so making a struct for future
    // proofing
    pub applications: Vec<Application>,
}

pub struct FilteredApplicationList {
    pub application_tuples: Vec<(Application, usize)>,
}

// need to create a struct represeting an individual app which will wrap a string and also
// have a bool which tracks whether the user has selected it from the list or not
#[derive(Clone)]
pub struct Application {
    pub name: String,
    pub status: AppSelectionStatus,
}

#[derive(Clone, Copy)]
pub enum AppSelectionStatus {
    Selected,
    NotSelected,
}

//TODO:
//if possible, combine init and build app list into one function.

// Solely for App initialization, build_app_list will be updating the list
// regularly.
pub fn init_app_list() -> Result<ApplicationList, io::Error> {
    let search_path;
    match env::consts::OS {
        "linux" => {
            search_path = Path::new("/usr/bin/");
        }
        _ => {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                "Operating system either not supported by this program, or failed to be detected.",
            ));
        }
    }

    let mut app_vec: Vec<Application> = Vec::new();

    for result in fs::read_dir(search_path)? {
        let result = String::from(result?.file_name().to_str().unwrap()); // if result exists unwrap
        app_vec.push(Application {
            name: result,
            status: AppSelectionStatus::NotSelected,
        })
    }

    Ok(ApplicationList {
        applications: app_vec,
    })
}

// shouldn't be public in the long term
pub fn filter_app_list(app: &App, search_state: &TextState) -> FilteredApplicationList {
    // TODO:
    // This never gets run until the user searches, so there won't be a
    // list displayed until the user searches something. Correct this behavior.

    let mut filtered_vec: Vec<(Application, usize)> = vec![];

    let master_list_len = app.application_list.applications.len();
    for i in 0..master_list_len {
        if app.application_list.applications[i]
            .name
            .contains(search_state.value())
            || search_state.value() == ""
        // if search state is empty include everything
        {
            filtered_vec.push((app.application_list.applications[i].clone(), i));
        }
    }

    FilteredApplicationList {
        application_tuples: filtered_vec,
    }
}
