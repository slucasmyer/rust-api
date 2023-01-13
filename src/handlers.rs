use std::convert::Infallible;
//use warp;
use warp::{self, http::StatusCode};


use crate::models::Customer;
use crate::db::Db;

pub async fn list_customers(db: Db) -> Result<impl warp::Reply, Infallible> {
    let customers = db.lock().await;
    //let customers: Vec<Customer> = customers.clone();
    //Ok(warp::reply::json(&customers))
    Ok(warp::reply::json(&*customers))

}


pub async fn get_customer(db: Db, guid: String) -> Result<impl warp::Reply, Infallible> {
    let customers = db.lock().await;
    let customer = customers.iter().find(|c| c.guid == guid);
    match customer {
        Some(c) => Ok(B)
        Some(c) => Ok(warp::reply::json(&c)),
        None => Ok(StatusCode::NOT_FOUND),
    }
}

pub async fn create_customer(db: Db, new_customer: Customer) -> Result<impl warp::Reply, Infallible> {
    let mut customers = db.lock().await;
    for customer in customers.iter() {
        if customer.guid == new_customer.guid {
            return Ok(StatusCode::CONFLICT);
        }
    }
    customers.push(new_customer);
    Ok(StatusCode::CREATED)
}

pub async fn update_customer(db: Db, guid: String, updated_customer: Customer) -> Result<impl warp::Reply, Infallible> {
    let mut customers = db.lock().await;
    let customer = customers.iter_mut().find(|c| c.guid == guid);
    match customer {
        Some(c) => {
            c = updated_customer;
            //c.first_name = updated_customer.first_name;
            //c.last_name = updated_customer.last_name;
            //c.email = updated_customer.email;
            //c.address = updated_customer.address;
            Ok(StatusCode::OK)
        },
        None => Ok(StatusCode::NOT_FOUND),
    }
}

pub async fn delete_customer(db: Db, guid: String) -> Result<impl warp::Reply, Infallible> {
    let mut customers = db.lock().await;
    let customer = customers.iter().position(|c| c.guid == guid);
    match customer {
        Some(c) => {
            customers.remove(c);
            Ok(StatusCode::OK)
        },
        None => Ok(StatusCode::NOT_FOUND),
    }
}