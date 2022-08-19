use actix_web::Responder;

pub async fn get_home() -> impl Responder {
    format!("API[V1.0] Home")
}

pub async fn login_user() -> impl Responder {
    format!("Logging User inn")
}

pub async fn register_user() -> impl Responder {
    format!("Register User")
}

pub async fn logout_user() -> impl Responder {
    format!("LogOut")
}

pub async fn add_task() -> impl Responder {
    format!("Add task")
}
pub async fn get_tasks() -> impl Responder {
    format!("Tasks")
}
pub async fn get_task_by_id() -> impl Responder {
    format!("Task by id")
}

pub async fn delete_task() -> impl Responder {
    format!("Delete Task")
}