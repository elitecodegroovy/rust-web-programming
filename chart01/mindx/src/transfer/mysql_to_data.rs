#[allow(unused)]
use sqlx::{MySql, Pool};

#[allow(unused_variables)]
pub async fn mysql_to_data(db_uri: &str) -> anyhow::Result<()> {
    // std::env::set_var("RUST_LOG", "info,sqlx::query=error");
    // let option = MySqlConnectOptions::from_str(db_uri)?;
    // let pool = MySqlPoolOptions::new()
    //     .max_connections(2)
    //     .connect_with(option)
    //     .await?;
    // let writer_actor = init_writer_actor(data_file);
    // let mut table_seq = TableSeq::default();
    // apply_config(&pool, &mut table_seq, &writer_actor).await?;
    // apply_tenant(&pool, &writer_actor).await?;
    // apply_user(&pool, &writer_actor).await?;
    Ok(())
}
#[allow(unused)]
async fn apply_config(
    pool: &Pool<MySql>,
) -> anyhow::Result<()> {
    // let mut config_pool = pool.clone();
    // let mut config_history_pool = pool.clone();
    // let mut config_dao = ConfigInfoDao::new(MySqlExecutor::new_by_pool(&mut config_pool));
    // let mut config_history_dao =
    //     ConfigHistoryDao::new(MySqlExecutor::new_by_pool(&mut config_history_pool));
    // let mut count = 0;
    // let mut offset = 0;
    // let limit = 100;
    // let mut config_query_param = ConfigInfoParam {
    //     id: None,
    //     limit: Some(limit),
    //     offset: Some(offset),
    // };
    // let mut patch = config_dao.query(&config_query_param).await?;
    // let mut patch_is_empty = patch.is_empty();
    //
    // log::info!("transfer config total count:{count}");    // let mut config_pool = pool.clone();
    // let mut config_history_pool = pool.clone();
    // let mut config_dao = ConfigInfoDao::new(MySqlExecutor::new_by_pool(&mut config_pool));
    // let mut config_history_dao =
    //     ConfigHistoryDao::new(MySqlExecutor::new_by_pool(&mut config_history_pool));
    // let mut count = 0;
    // let mut offset = 0;
    // let limit = 100;
    // let mut config_query_param = ConfigInfoParam {
    //     id: None,
    //     limit: Some(limit),
    //     offset: Some(offset),
    // };
    // let mut patch = config_dao.query(&config_query_param).await?;
    // let mut patch_is_empty = patch.is_empty();
    //
    // log::info!("transfer config total count:{count}");
    Ok(())
}
