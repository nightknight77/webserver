mod http;
mod serverimpl;

use crate::serverimpl::Server;
use crate::serverimpl::ThreadPool;

fn main() {

    let server =
        Server::new("127.0.0.1:7877".to_string(), ThreadPool::new(4));
    server.run();
}
