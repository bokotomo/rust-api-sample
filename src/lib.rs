#[macro_use]
extern crate diesel;

pub mod domain;
pub mod controller;
pub mod response;
pub mod request;
pub mod repository;
pub mod service;
pub mod driver;
pub mod model;
pub mod schema;

#[cfg(test)]
mod tests;