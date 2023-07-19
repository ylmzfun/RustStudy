use crate::schema::student::dsl::student;
use crate::schema::student::{title as title_str};
use crate::schema::class::title as c_t_s;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;
use crate::models::{Class, NewClass};
use crate::schema::class::dsl::class;

pub mod models;
pub mod schema;

// 创建连接
pub fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

use self::models::{NewStudent, Student};
// 创建学生
pub fn create_stu_m(conn: &mut PgConnection, title: &str, class_id: &i32, club_id: &i32) -> Student {
    use crate::schema::student;
    let new_student = NewStudent {
        title,
        class_id,
        club_id,
    };

    diesel::insert_into(student::table)
        .values(&new_student)
        .returning(Student::as_returning())
        .get_result(conn)
        .expect("Error saving new post")
}

// 学生列表
pub fn list_stu_m(conn: &mut PgConnection) {
    let results = student
        .limit(5)
        .select(Student::as_select())
        .load(conn)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
    for post in results {
        println!("{}", post.title);
        println!("-----------\n");
        println!("{}", post.class_id);
    }
}

// 删除学生
pub fn delete_stu_m(conn: &mut PgConnection, title_s: &str) {
    let pattern = format!("%{}%", title_s);
    let num_deleted = diesel::delete(student.filter(title_str.like(pattern)))
        .execute(conn)
        .expect("Error deleting posts");

    println!("Deleted {} posts", num_deleted);
}

// 更新学生
pub fn update_stu_m(conn: &mut PgConnection, id: i32, title_s: &str) {
    let post = diesel::update(student.find(id))
        .set(title_str.eq(title_s))
        .returning(Student::as_returning())
        .get_result(conn)
        .unwrap();
    println!("Published post {}", post.title);
}


// 创建班级
pub fn create_class(conn: &mut PgConnection, title: &str) -> Class {
    use crate::schema::class;
    let new_class = NewClass {
        title,
    };

    diesel::insert_into(class::table)
        .values(&new_class)
        .returning(Class::as_returning())
        .get_result(conn)
        .expect("Error saving new post")
}

// 删除班级
pub fn delete_class(conn:&mut PgConnection, class_title_str:&str){
    let d_num = diesel::delete(class.filter(c_t_s.like(class_title_str))).execute(conn).expect("Error deleting posts");
    println!("Deleted {} posts", d_num);
}

// 班级列表
pub fn list_class(conn:&mut PgConnection){
    let results = class
        .limit(5)
        .select(Class::as_select())
        .load(conn)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
    for cla in results {
        println!("{}", cla.title);
        println!("-----------\n");
        println!("{}", cla.id);
    }
}

// 班级更新
pub fn update_class(conn:&mut PgConnection, id:i32,title: &str){
    let post = diesel::update(class.find(id))
        .set(c_t_s.eq(title))
        .returning(Class::as_returning())
        .get_result(conn)
        .unwrap();
    println!("Published post {}", post.title);
}


#[cfg(test)]
mod test {
    use crate::{create_class, create_stu, delete_class, delete_stu, establish_connection, list_class, list_stu, update_class, update_stu};

    #[test]
    fn db_test() {
        let connection = &mut establish_connection();

        // 创建学生
        // let name = String::from("xiaoming1111");
        // let class_id: i32 = 1;
        // let club_id: i32 = 1;
        // let stu = create_stu(connection, &name, &class_id,&club_id);
        // print!("{:?}", stu);
        // println!("\nSaved draft {} with id {}", name, stu.id);

        // 学生列表
        // list_stu(connection)

        // 删除学生
        // delete_stu(connection,"xiaoming1111")

        // 更新学生
        // update_stu(connection, 1,"ddddddd")

        // 创建班级
        // let title = String::from("二班");
        // let cla = create_class(connection, &title);
        // print!("{:?}", cla);
        // println!("\nSaved draft {} with id {}", title, cla.id);

        // 删除班级
        // delete_class(connection, "二班")

        // 班级列表
        // list_class(connection)

        // 修改班级
        // update_class(connection, 1,"小一班")
    }
}
