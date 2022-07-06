//! Microsoft SQL (MSSQL) database driver.

use crate::executor::Executor;

pub mod arguments;
pub mod column;
pub mod connection;
pub mod database;
pub mod error;
pub mod io;
pub mod options;
pub mod protocol;
pub mod query_result;
pub mod row;
pub mod statement;
pub mod transaction;
pub mod type_info;
pub mod types;
pub mod value;

pub use arguments::MssqlArguments;
pub use column::MssqlColumn;
pub use connection::MssqlConnection;
pub use database::Mssql;
pub use error::MssqlDatabaseError;
pub use options::MssqlConnectOptions;
pub use query_result::MssqlQueryResult;
pub use row::MssqlRow;
pub use statement::MssqlStatement;
pub use transaction::MssqlTransactionManager;
pub use type_info::MssqlTypeInfo;
pub use value::{MssqlValue, MssqlValueRef};

/// An alias for [`Pool`][crate::pool::Pool], specialized for MSSQL.
pub type MssqlPool = crate::pool::Pool<Mssql>;

/// An alias for [`PoolOptions`][crate::pool::PoolOptions], specialized for MSSQL.
pub type MssqlPoolOptions = crate::pool::PoolOptions<Mssql>;

/// An alias for [`Executor<'_, Database = Mssql>`][Executor].
pub trait MssqlExecutor<'c>: Executor<'c, Database = Mssql> {}
impl<'c, T: Executor<'c, Database = Mssql>> MssqlExecutor<'c> for T {}

// NOTE: required due to the lack of lazy normalization
impl_into_arguments_for_arguments!(MssqlArguments);
impl_executor_for_pool_connection!(Mssql, MssqlConnection, MssqlRow);
impl_executor_for_transaction!(Mssql, MssqlRow);
impl_acquire!(Mssql, MssqlConnection);
impl_column_index_for_row!(MssqlRow);
impl_column_index_for_statement!(MssqlStatement);
impl_into_maybe_pool!(Mssql, MssqlConnection);
