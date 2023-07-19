use actix_web::{App, get, HttpResponse, HttpServer, post, Responder, web};
use diesel::{PgConnection, QueryDsl, RunQueryDsl};
use l3::{create_stu_m, delete_stu_m, establish_connection, list_stu_m, update_stu_m};
use l3::models::Student;
use l3::schema::student::dsl::student;
use diesel::prelude::*;
use l3::schema::club::title;

pub mod models;
pub mod schema;

#[get("/stu/list")]
async fn list_stu() -> impl Responder {
    let connection = &mut establish_connection();
    list_stu_m(connection);
    HttpResponse::Ok().body("111")
}

#[get("/create/stu")]
async fn create_stu() -> impl Responder {
    let connection = &mut establish_connection();

    let name = String::from("xiaoming1111");
    let class_id: i32 = 1;
    let club_id: i32 = 1;
    let stu = create_stu_m(connection, &name, &class_id,&club_id);
    HttpResponse::Ok().body(eprint!("\nSaved draft {} with id {}", name, stu.id))
}

#[get("/update/stu")]
async fn update_stu() -> impl Responder {
    let connection = &mut establish_connection();
    update_stu_m(connection,1,"xiaoming1111");
    HttpResponse::Ok().body("update_stu")
}

#[post("/delete/stu")]
async fn delete_stu() -> impl Responder {
    let connection = &mut establish_connection();
    delete_stu_m(connection,"xiaoming1111");
    HttpResponse::Ok().body("delete_stu")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(list_stu)
            .service(create_stu)
            .service(update_stu)
            .service(delete_stu)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
