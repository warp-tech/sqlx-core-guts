//! **PostgreSQL** database driver.

use crate::executor::Executor;

pub mod advisory_lock;
pub mod arguments;
pub mod column;
pub mod connection;
pub mod copy;
pub mod database;
pub mod error;
pub mod io;
pub mod listener;
pub mod message;
pub mod options;
pub mod query_result;
pub mod row;
pub mod statement;
pub mod transaction;
pub mod type_info;
pub mod types;
pub mod value;

#[cfg(feature = "migrate")]
pub mod migrate;

pub use advisory_lock::{PgAdvisoryLock, PgAdvisoryLockGuard, PgAdvisoryLockKey};
pub use arguments::{PgArgumentBuffer, PgArguments};
pub use column::PgColumn;
pub use connection::PgConnection;
pub use copy::PgCopyIn;
pub use database::Postgres;
pub use error::{PgDatabaseError, PgErrorPosition};
pub use listener::{PgListener, PgNotification};
pub use message::PgSeverity;
pub use options::{PgConnectOptions, PgSslMode};
pub use query_result::PgQueryResult;
pub use row::PgRow;
pub use statement::PgStatement;
pub use transaction::PgTransactionManager;
pub use type_info::{PgTypeInfo, PgTypeKind};
pub use types::PgHasArrayType;
pub use value::{PgValue, PgValueFormat, PgValueRef};

/// An alias for [`Pool`][crate::pool::Pool], specialized for Postgres.
pub type PgPool = crate::pool::Pool<Postgres>;

/// An alias for [`PoolOptions`][crate::pool::PoolOptions], specialized for Postgres.
pub type PgPoolOptions = crate::pool::PoolOptions<Postgres>;

/// An alias for [`Executor<'_, Database = Postgres>`][Executor].
pub trait PgExecutor<'c>: Executor<'c, Database = Postgres> {}
impl<'c, T: Executor<'c, Database = Postgres>> PgExecutor<'c> for T {}

impl_into_arguments_for_arguments!(PgArguments);
impl_executor_for_pool_connection!(Postgres, PgConnection, PgRow);
impl_executor_for_transaction!(Postgres, PgRow);
impl_acquire!(Postgres, PgConnection);
impl_column_index_for_row!(PgRow);
impl_column_index_for_statement!(PgStatement);
impl_into_maybe_pool!(Postgres, PgConnection);
impl_encode_for_option!(Postgres);
