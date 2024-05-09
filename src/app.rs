use axum::{response::Html, routing::get, Extension, Router};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use tokio::net::TcpListener;

use crate::{routes::{city_route::get_city_router, country_route::get_country_router, state_route::get_state_router}, service_register::ServiceRegister};


#[derive(Clone)]
pub struct AppState {
    pub db_pool: Pool<Postgres>
}


#[derive(Clone)]
pub struct ServerInfo {
    pub ip : [u8;4],
    pub port : u16,
}


impl ServerInfo {
    pub fn new(ip : [u8;4],port :u16) -> ServerInfo
    {
        Self { ip, port }
    }
}

#[derive(Debug)]
pub enum AppStateError {
    DbConnectionFailed
}

impl AppState {
    pub async fn new(conn : &str,pool_size : u32) -> Result<AppState,AppStateError> {
        let pool = PgPoolOptions::new()
        .max_connections(pool_size)
        .connect(conn)
        .await
        .map_err(|e| AppStateError::DbConnectionFailed);

        if pool.is_err(){
            return Err(pool.err().unwrap());
        }

        Ok(Self {
            db_pool : pool.ok().unwrap()
        })
    }

    pub fn from_db(pool:Pool<Postgres>) -> AppState {
        Self { db_pool: pool }
    }

}

pub struct App {
    server_info:ServerInfo,
    app_state : AppState,
    service_register : ServiceRegister   

}

#[derive(Debug)]
pub enum AppError {
    AppUnexpectedError
}

impl App {
    pub fn new(ip_ref : &str,port :u16,app_state : AppState,service_register : ServiceRegister) -> App {
        let parts = ip_ref.split('.');
        let mut ip : [u8;4] = Default::default();
        for (i,part) in parts.into_iter().enumerate()
        {
            ip[i] = part.parse().expect("Given ip address is not valid");
        }
        Self {
            server_info : ServerInfo::new(ip,port),
            app_state,
            service_register
        }
    }

    pub fn from_port(port:u16,app_state : AppState,service_register : ServiceRegister) -> App {
        let ip = [127,0,0,1];
        Self {
            server_info : ServerInfo::new(ip, port),
            app_state,
            service_register
        }
    }

    
    pub async fn run(&self) -> Result<(),AppError> {
        // Create router
        let app = create_router(self.app_state.clone(),self.server_info.clone(),self.service_register.clone());
        let server_info = &self.server_info;

        let addr = format!("{}.{}.{}.{}",server_info.ip[0],server_info.ip[1],server_info.ip[2],server_info.ip[3]);
        
        let listener = TcpListener::bind(format!("{addr}:{}",server_info.port)).await.unwrap();

        println!("Listening on {addr}:{}",server_info.port);
        let res = axum::serve(listener,app).await.map_err(|e| AppError::AppUnexpectedError);

        if res.is_err(){
            return Err(res.err().unwrap());
        }

        Ok(res.ok().unwrap())
    }
}

fn create_router(app_state : AppState,server_info : ServerInfo,service_register : ServiceRegister) -> Router {
    Router::new()
        .with_state(app_state)
        .route("/",get(|| async { Html("Welcome to my api")  }))
        .nest("/api/country",get_country_router())
        .nest("/api/city",get_city_router())
        .nest("/api/state",get_state_router())
        .layer(Extension(service_register.city_repo))
        .layer(Extension(service_register.country_repo))
        .layer(Extension(service_register.state_repo))
        .layer(Extension(server_info))

}