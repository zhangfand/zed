use std::borrow::Cow;

#[derive(Debug)]
struct TestDb;

struct TestConnection(rusqlite::Connection);
struct TestTransactionManager;
struct TestRow;
struct TestQueryResult;
struct TestTypeInfo;
struct TestColumn;
struct TestValue;

#[derive(Default)]
struct TestArguments;

enum TestArgumentValue<'q> {
    Null,
    Text(Cow<'q, str>),
    Blob(Cow<'q, [u8]>),
    Double(f64),
    Int(i32),
    Int64(i64),
}

impl<'q> sqlx::Arguments<'q> for TestArguments {
    type Database = TestDb;

    fn reserve(&mut self, additional: usize, size: usize) {
        todo!()
    }

    fn add<T>(&mut self, value: T)
    where
        T: 'q + Send + sqlx::Encode<'q, Self::Database> + sqlx::Type<Self::Database>,
    {
        todo!()
    }
}

impl<'q> sqlx::database::HasArguments<'q> for TestDb {
    type Database = TestDb;
    type Arguments = TestArguments;
    type ArgumentBuffer = Vec<TestArgumentValue<'q>>;
}

impl<'q> sqlx::database::HasStatement<'q> for TestDb {}

impl<'q> sqlx::database::HasValueRef<'q> for TestDb {}

impl sqlx::Database for TestDb {
    type Connection = TestConnection;
    type TransactionManager = TestTransactionManager;
    type Row = TestRow;
    type QueryResult = TestQueryResult;
    type Column = TestColumn;
    type TypeInfo = TestTypeInfo;
    type Value = TestValue;
}
