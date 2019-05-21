pub mod routes {
    use router::Router;
    use crate::handlers::handlers;
    use crate::storage::storage;

    pub fn app_router() -> Router {
        let mut router = Router::new();

        let st = storage::Mock::new();
        let handlers = handlers::Handlers::new(st);

        router.get("/", handlers.index_handler, "index");

        router
    }
}

