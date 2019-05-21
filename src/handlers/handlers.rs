extern crate iron;
extern crate router;

use iron::{Request, Response, IronResult};
use iron::status;
use log::info;
use iron::middleware::Handler;

use crate::storage::storage;
use crate::storage::storage::Storage;

pub struct IndexHandler {
    storage: storage::Mock,
}

impl IndexHandler {
    pub fn new(st: storage::Mock) -> IndexHandler {
        IndexHandler{storage: st}
    }
}
impl Handler for IndexHandler {
    fn handle(&self, _: &mut Request) -> IronResult<Response> {
        let v = self.storage.get();
        info!("handle index");
        Ok(Response::with((status::Ok, format!("Hello world with value: {}", v))))
    }
}

pub struct Handlers {
    pub index_handler: IndexHandler,
}

impl Handlers {
    pub fn new(st: storage::Mock) -> Handlers {
        Handlers {
            index_handler: IndexHandler::new(st),
        }
    }
}
