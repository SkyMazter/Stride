// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use serde::Serialize;
use serde_json::{json, Value};

#[derive(Serialize)]
struct Task {
    id: u32,
    title: String,
    is_complete: bool,
}

#[derive(Serialize)]
struct Project {
    id: u32,
    title: String,
    tasks: Vec<Task>,
    percentage: u32,
    is_complete: bool,
}

// use std::collections::HashMap;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn get_all_projects() -> Value {
    let sample_projects: Vec<Project> = vec![
        Project {
            id: 1,
            title: "Planning App".into(),
            tasks: vec![Task {
                id: 101,
                title: "Desing UI".into(),
                is_complete: false,
            }],
            percentage: 0,
            is_complete: false,
        },
        Project {
            id: 2,
            title: "Build a Bird House".into(),
            tasks: vec![
                Task {
                    id: 201,
                    title: "Get Supplies".into(),
                    is_complete: false,
                },
                Task {
                    id: 202,
                    title: "Cut Wood".into(),
                    is_complete: false,
                },
                Task {
                    id: 203,
                    title: "Mount Base".into(),
                    is_complete: false,
                },
            ],
            percentage: 20,
            is_complete: false,
        },
    ];

    json!(sample_projects)
}

// #[tauri::command]
// fn update_task_states(state: TaskStatePayload) {
//     println!("{:#?}", state);
//     // save to DB, file, or log
// }

#[tauri::command]
fn update(project_id: u32, task_id: u32, is_checked: bool) {
    println!(
        "Project {} Task {} is now {}",
        project_id, task_id, is_checked
    );
    // Save to DB, update a file, etc.
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, get_all_projects, update])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
