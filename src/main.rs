#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate log;
#[macro_use] extern crate rocket_contrib;
extern crate rusqlite;


use rocket_contrib::databases::rusqlite::Connection;
mod domain;

use std::collections::HashMap;

use rocket::Request;
use rocket_contrib::templates::Template;

#[database("sqlite_database")]
pub struct DbConn(Connection);

#[derive(Serialize)]
struct TemplateContext {
    name : String,
    error_report : Option<domain::ErrorReport>
}

#[get("/")]
fn index() -> &'static str {
    "Templating Example"
}

#[get("/hello/<name>")]
fn get(name: String) -> Template{
 info!("coucou xavier");
 let appName = "xw";
 let error = domain::build_error_report(&appName, domain::Status::CREATED);
 let context = TemplateContext{name,error_report:Some(error)};
 Template::render("index", &context)
}



#[catch(404)]
fn not_found(req:&Request<'_>) -> Template{
    let mut map = HashMap::new();
    map.insert("path",req.uri().path());    
    Template::render("error/404",&map)
}

fn main() {
    
    rocket::ignite()
    .mount("/", routes![index,get])
    .attach(Template::fairing())
    .attach(DbConn::fairing())
    .register(catchers![not_found]).launch();
}



