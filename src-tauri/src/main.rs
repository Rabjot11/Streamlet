// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

//imports
use api::repository;
use reqwest::Client;
use serde_json::{json, Value};

fn main() {

  // main start of application
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_data])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
//https://tauri.app/v1/guides/features/command/
async fn get_data(api_type: String) -> Result<Value, ()> {

    //client for api callings
    let client: Client = reqwest::Client::new();
    match api_type.as_str() {
        "homepage" => {
            println!("Getting Homepage...");
            let value: Value = repository::homepage::homepage::trending_movies(client).await;
            Result::Ok(value)
            // print!("Getting");
        }

        //Getting invalid request from front-end side
        _ => Result::Ok(json!({
            "status": "error",
            "data": "Invalid api type",
        })),
    }
}

// ------------------------------Module imports--------------------------------

//imports
//crates intialization
// all modules should be initialized here
mod api {
    // pub mod api_initialize;

    // Repository for api callings
    // All api callings will be called from here
    pub mod repository {

      //all apis
        pub mod homepage;
    }

    // Constant modules for api-calling etc
    pub mod constants {
        pub mod endpoints;
    }

    // Error handling module for api callings
    pub mod error_handling {
        pub mod error;
    }

    pub mod helper {
        pub mod api_response;
    }
}
