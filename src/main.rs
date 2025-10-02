use axum::{
    extract::{State,Json},
    routing::{get,post},
    Router,
};
use serde::{Serialize,Deserialize};
use std::sync::{Arc,Mutex};
use std::net::SocketAddr;
use tokio ::net::TcpListener;

#[derive(Serialize, Clone)]
struct User{
    id:u32,
    name:String,
}

#[derive(Deserialize)]
struct NewUser{
    name:String,
}

#[derive(Clone)]
struct AppState{
    users:Arc<Mutex<Vec<User>>>,
}

async fn list_users(State(state):State<AppState>) -> Json<Vec<User>> {
    let users = state.users.lock().unwrap();
    Json(users.clone())
}

async fn add_user(State(state):State<AppState>,Json(payload):Json<NewUser>)->Json<User>{
    let mut user=state.users.lock().unwrap();
    let new_id=(user.len() as u32)+1;
    let new_user=User{
        id:new_id,
        name:payload.name,
    };
    user.push(new_user.clone());
    Json(new_user)
}

#[tokio::main]
async fn main() {
    let state=AppState{
        users:Arc::new(Mutex::new(vec![
            User{id:1,name:"zeynab".to_string()},
            User{id:2,name:"zahra".to_string()},
        ])),
    };
    let app=Router::new()
        .route("/users",get(list_users).post(add_user))
        .with_state(state);

    let addr=SocketAddr::from(([127,0,0,1],3000));

    println!("server running on http://{}",addr);

    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener,app).await.unwrap();
}
