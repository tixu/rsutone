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
    reports: Vec<domain::ErrorReport>,
}

#[get("/")]
fn index() -> &'static str {
    "Templating Example"
    
}
#[get("/metrics")]
fn metrics(db: DbConn) -> Template{ 
    #[derive(Serialize)]  
    struct MetricsContext  {
        name : String,
        case_numbers: i32
    }
    
    let result:Result<i32,_> =  db.query_row("SELECT count(1) description FROM reports",&[],|row| {row.get(0)});
    let number = match result {
        Ok(number) => number,
        Err(e) => 0
    };
    info!("number of rows : {}", number);
    let context = MetricsContext{name: String::from("eee"),case_numbers:number};
    Template::render("metrics", &context)
}


#[get("/reports")]
fn reports(db: DbConn) -> Template{ 
    let mut stmt = db.prepare("SELECT id, app_name, app_version, general_info,error, error_status from reports").unwrap();
    let error_reports = stmt.query_map(&[], |row| {
         let context: domain::Context = domain::Context{
                    application_name: row.get(1),
                    application_version: row.get(2),
                    general_info : row.get(3)

               };
         let error: domain::Error = domain::Error{
                    error: row.get(4),
         };
        domain::ErrorReport{
            id: row.get(0),
            context: context,
            error: error,
            status: domain::Status::SOLVED
         }
    }).unwrap().map(|er| er.unwrap());
    let context:TemplateContext = TemplateContext{name:String::from("eee"),reports: error_reports.collect()};
    Template::render("reports", &context)
}

#[catch(404)]
fn not_found(req:&Request<'_>) -> Template{
    let mut map = HashMap::new();
    map.insert("path",req.uri().path());    
    Template::render("error/404",&map)
}

fn main() {
    rocket::ignite()
    .mount("/", routes![index,reports,metrics])
    .attach(Template::fairing())
    .attach(DbConn::fairing())
    .register(catchers![not_found]).launch();
}



