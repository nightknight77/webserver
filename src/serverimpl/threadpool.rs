
pub struct ThreadPool {
    pool_size: usize
    // vec of threads
}

impl ThreadPool {

    pub fn new(pool_size: usize) -> Self{
        ThreadPool {
            pool_size
        }
    }
}