use std::error::Error;
use std::sync::Arc;
use mindx::common::{get_app_version, get_app_log_level, AppSysConfig};
use env_logger_timezone_fmt::{TimeZoneFormat, TimeZoneFormatEnv};
use env_logger::TimestampPrecision;
use mindx::cli::{Commands};
use clap::Parser;
use mindx::cli;
use mindx::transfer::mysql_to_data::mysql_to_data;

pub async fn bootstrap() -> Result<(), Box<dyn Error>>{
    let cli_opt = cli::Cli::parse();
    let env_path = &cli_opt.env_file;
    //let env_path = std::env::var("MINDX_ENV_FILE").unwrap_or_default();
    if env_path.is_empty() {
        dotenv::dotenv().ok();
    } else {
        dotenv::from_path(env_path).ok();
    }
    // init rust log
    unsafe { init_rust_log(); }

    //init zone
    let sys_config = Arc::new(AppSysConfig::init_from_env());
    let timezone_fmt = Arc::new(TimeZoneFormatEnv::new(
        sys_config.gmt_fixed_offset_hours.map(|v| v * 60 * 60),
        Some(TimestampPrecision::Micros),
    ));
    env_logger::Builder::from_default_env()
        .format(move |buf, record| TimeZoneFormat::new(buf, &timezone_fmt).write(record))
        .init();
    if let Some(cmd) = cli_opt.command {
        return run_sub_cmd(cmd).await;
    }
    log::info!("system version:{}, RUST_LOG level:{}", get_app_version(), get_app_log_level());
    log::info!("system data dir:{}", sys_config.local_db_dir);

    Ok(())
}

unsafe fn init_rust_log() {
    let rust_log = std::env::var("RUST_LOG").unwrap_or("info".to_owned());
    std::env::set_var("RUST_LOG", &rust_log);
}



async fn run_sub_cmd(commands: Commands) -> Result<(), Box<dyn Error>> {
    match commands {
        Commands::MysqlToData { uri, out } => {
            log::info!("nacos mysql to middle data, from: mysql://** to:{out}");
            mysql_to_data(&uri).await?;
        }
        _ => {}
    }
    Ok(())
}