use std::convert::Infallible;
use warp::{self, Filter};

use crate::db::Db;
use crate::handlers;
use crate::models::Customer;

pub fn routes(db: Db) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let list_customers = warp::path!("customers")
        .and(warp::get())
        .and(with_db(db.clone()))
        .and_then(handlers::list_customers);

    let get_customer = warp::path!("customers" / String)
        .and(warp::get())
        .and(with_db(db.clone()))
        .and_then(handlers::get_customer);

    let create_customer = warp::path!("customers")
        .and(warp::post())
        .and(warp::body::json())
        .and(with_db(db.clone()))
        .and_then(handlers::create_customer);

    let update_customer = warp::path!("customers" / String)
        .and(warp::put())
        .and(warp::body::json())
        .and(with_db(db.clone()))
        .and_then(handlers::update_customer);

    let delete_customer = warp::path!("customers" / String)
        .and(warp::delete())
        .and(with_db(db.clone()))
        .and_then(handlers::delete_customer);

    get_customer
        .or(list_customers)
        .or(create_customer)
        .or(update_customer)
        .or(delete_customer)
        .with(warp::log("customers"))
}

fn with_db(db: Db) -> impl Filter<Extract = (Db,), Error = Infallible> + Clone {
    warp::any().map(move || db.clone())
}
