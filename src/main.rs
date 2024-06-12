#![allow(dead_code)]
#![allow(unused_imports)]

use axum::{
  extract::Json,
  response::Json as ResponseJson,
  routing::{get, post},
  Router,
};
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use std::process::{Command, Stdio};
use std::{env, fs};

#[derive(Deserialize, Debug)]
struct JobInput {
  asm: String,
}

#[derive(Serialize, Debug)]
struct JobOutput {
  exit_code: i32,
  stdout: String,
  stderr: String,
}

async fn handle_job(Json(payload): Json<JobInput>) -> ResponseJson<JobOutput> {
  let fileext = env::var("ASM_FILE_EXT").unwrap();
  let assemble_link_cmd = env::var("ASSEMBLE_LINK_EXEC_CMD").unwrap();
  let filename = format!("program.{}", fileext);
  clean_files(&filename);

  fs::write(&filename, payload.asm).unwrap();

  let output = Command::new("sh")
    .arg("-c")
    .arg(assemble_link_cmd)
    .stdout(Stdio::piped())
    .stderr(Stdio::piped())
    .output()
    .expect("failed to execute process");

  let stdout = String::from_utf8(output.stdout).unwrap();
  let stderr = String::from_utf8(output.stderr).unwrap();
  let exit_code = output.status.code().unwrap();
  clean_files(&filename);

  ResponseJson(JobOutput { exit_code, stdout, stderr })
}

#[tokio::main]
async fn main() {
  dotenv().ok();

  // build our application with a single route
  let app = Router::new()
    .route("/foo", post(handle_job))
    .route("/", get(|| async { "Hello, World!" }));

  // run our app with hyper, listening globally on port 3000
  let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
    .await
    .unwrap();

  println!("listening on {}", listener.local_addr().unwrap());
  axum::serve(listener, app).await.unwrap();
}

fn clean_files(filename: &str) {
  _ = fs::remove_file("obj.o");
  _ = fs::remove_file("a.out");
  _ = fs::remove_file("program.asm");
  _ = fs::remove_file("program.s");
  _ = fs::remove_file(filename);
}
