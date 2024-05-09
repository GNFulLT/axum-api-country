use sqlx::{postgres::PgRow, query_as, FromRow};

use super::{r#abstract::error::{RepositoryError, RepositoryResult}, PostgresPool};

pub async fn get_t_pagination<'query,T>(pool : &PostgresPool,select_op : &str,table_name :&str,skip :u32,count : u32) -> RepositoryResult<Vec<T>>
where
T: Send + Unpin + for<'r> FromRow<'r, PgRow>,
{
       query_as::<_,T>(
            &format!("SELECT {} FROM {} LIMIT {} OFFSET {};",select_op,table_name,count,skip))
              .fetch_all(pool)
              .await
              .map_err(|e| RepositoryError::QueryExecutionError(e.to_string()))
}

pub async fn get_t_option_where<'query,T>(pool : &PostgresPool,select_op : &str,table_name :&str,where_op:&str) -> RepositoryResult<Option<T>>
where
T: Send + Unpin + for<'r> FromRow<'r, PgRow>,
{
       query_as::<_,T>(
            &format!("SELECT {} FROM {} WHERE {};",select_op,table_name,where_op))
              .fetch_optional(pool)
              .await
              .map_err(|e| RepositoryError::QueryExecutionError(e.to_string()))
}
