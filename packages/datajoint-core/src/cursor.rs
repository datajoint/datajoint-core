use sqlx::Executor;
// Used to execute queries and access their results.
// Wraps sqlx::Executor.
pub struct Cursor {
    executor: &impl sqlx::Executor;
}
impl Cursor {
    fn new() -> Cursor {
        Cursor {

        }
    }

    fn load_executor(&self, executor: &impl sqlx::Executor) {
        self.executor = executor;
    }

    // Fetches all of the query results.
    fn fetch_all(&self) -> TableRowVector{
        // fetch all query results and return a TableRowVector on success
        self.executor.fetch_all();
    }

    // Fetches the next result of the query.
    fn fetch_one(&self) -> TableRow {
        self.executor.fetch_one();
    }
}