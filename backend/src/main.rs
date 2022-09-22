#[macro_use]
extern crate rocket;
// use rocket::response::stream::{Event, EventStream};
use rocket::serde::json::Json;
use rocket::tokio::task;
// use rocket::tokio::time::{self, Duration};
use serde::Deserialize;
use ssh2::Session;
use std::fs;
use std::{io, net::TcpStream};
//DB
pub mod models;
pub mod schema;

// use self::models::{InstallationItem, NewInstallation};
use crate::models::*;
use crate::schema::*;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

#[get("/")]
fn dash() -> &'static str {
    "Working"
}


pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

async fn heavy_computation(option: String, textarea: String) {
    let (vpshost, vpsuser, vpspass, _domain, _domainhost, _domainuser, _domainpass) =
        string_parser(textarea);

    let option: &str = &option[..];
    let setup_file = match option {
        "setup1" => "setup1.txt",
        "setup2" => "setup2.txt",
        "setup3" => "setup3.txt",
        "setup4" => "setup4.txt",
        _ => "not valid",
    };
    let order_number = "None".to_string();
    let setup_name = option.to_string();
    let order_status = "Running".to_string();
    let username = "Shaheer".to_string();

    let ii = create_installation(order_number, setup_name, order_status, username, _domain);

    let file: String = fs::read_to_string(setup_file).unwrap();
    // Connect to the local SSH server
    let tcp = TcpStream::connect(format!("{vpshost}:22")).unwrap();
    let mut sess = Session::new().unwrap();
    sess.set_tcp_stream(tcp);
    sess.handshake().unwrap();
    sess.userauth_password(&vpsuser, &vpspass).unwrap();
    if sess.authenticated() {
        println!("Connected Successfully!!");
    }

    let commands = file.replace('\r', "");

    println!("Executing.........\n");

    let mut channel = sess.channel_session().unwrap();
    channel.request_pty("xterm", None, None).unwrap();
    channel
        .handle_extended_data(ssh2::ExtendedData::Merge)
        .unwrap();

    channel.exec(&commands).unwrap();

    let mut ssh_stdout = channel.stream(0);
    // let stdout = io::stdout();
    // let mut stdout = stdout.lock();
    // io::copy(&mut ssh_stdout, &mut stdout).unwrap();
    let mut writer: Vec<u8> = vec![];

    io::copy(&mut ssh_stdout, &mut writer).unwrap();

    channel.wait_close().unwrap();
    let exit_code = channel.exit_status().unwrap();

    println!("Exit code: {}\n", exit_code);

    let iid = ii.id;

    if exit_code == 0 {
        use crate::installation::dsl::*;
        let st = "done".to_string();
        let _updated_row = diesel::update(installation.filter(id.eq(&iid)))
            .set(order_status.eq(&st))
            .execute(&mut establish_connection())
            .expect("Error updating installation");
    }
}

fn string_parser(textarea: String) -> (String, String, String, String, String, String, String) {
    let collect: String = textarea
        .replace(' ', "")
        .replace('\n', ":")
        .replace('\r', "")
        .replace('\t', "")
        .replace(' ', "");

    let collect: Vec<&str> = collect.split(":").collect();

    let tup = (
        collect[1].to_string(),
        collect[3].to_string(),
        collect[5].to_string(),
        collect[7].to_string(),
        collect[9].to_string(),
        collect[11].to_string(),
        collect[13].to_string(),
    );
    // (vpshost,vpsuser,vpspass,domain,domainhost,domainuser,domainpass)
    tup
}

fn create_installation(
    order_number: String,
    setup_name: String,
    order_status: String,
    username: String,
    domain: String,
) -> InstallationItem {
    // use crate::schema::installation;
    let new_installation = NewInstallation {
        order_number,
        setup_name,
        order_status,
        username,
        domain,
    };
    let conn = &mut establish_connection();

    diesel::insert_into(installation::table)
        .values(&new_installation)
        .get_result(conn)
        .expect("Error saving new Installation in Database")
}

#[derive(Debug, PartialEq, Eq, Deserialize, FromForm)]
struct User {
    option: String,
    textarea: String,
}

#[post("/ffff", format = "json", data = "<user_input>")]
fn run_setup(user_input: Json<User>) {
    println!("print test {:?}", user_input);
    task::spawn(heavy_computation(
        user_input.option.clone(),
        user_input.textarea.clone(),
    ));
}

#[get("/homeee")]
fn home() -> Json<Vec<InstallationItem>> {
    let results: Vec<InstallationItem> = installation::table
        .select(installation::all_columns)
        .load::<InstallationItem>(&mut establish_connection())
        .expect("Error loading posts");

    Json(results)
}

#[delete("/delete/<iid>")]
fn delete(iid: i32) {
    println!("Deleted no.{:?} ", iid);
    use crate::installation::dsl::*;
    let num_deleted = diesel::delete(installation.filter(id.eq(&iid)))
        .execute(&mut establish_connection())
        .expect("Error deleting posts");

    println!("Deleted {} installation", num_deleted);
}
// #[get("/events")]
// fn stream() -> EventStream![] {
//     EventStream! {
//         let mut interval = time::interval(Duration::from_secs(1));
//         loop {
//             yield Event::data("ping");
//             interval.tick().await;
//         }
//     }
// }

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/api", routes![run_setup, dash, home, delete])
}
