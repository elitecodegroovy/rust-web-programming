
use super::common::{get_app_version, AppSysConfig};

pub fn init_env(env_path: &str) {
    //let env_path = std::env::var("MINDX_ENV_FILE").unwrap_or_default();
    if env_path.is_empty() {
        dotenv::dotenv().ok();
    } else {
        dotenv::from_path(env_path).ok();
    }
    // init rust log
    unsafe { init_rust_log(); }

    //init zone
    init_time_zone();
}

unsafe fn init_rust_log() {
    let rust_log = std::env::var("RUST_LOG").unwrap_or("info".to_owned());
    std::env::set_var("RUST_LOG", &rust_log);
}

fn init_time_zone() {
    let sys_config = Arc::new(AppSysConfig::init_from_env());
    let timezone_fmt = Arc::new(TimeZoneFormatEnv::new(
        sys_config.gmt_fixed_offset_hours.map(|v| v * 60 * 60),
        Some(TimestampPrecision::Micros),
    ));
    env_logger::Builder::from_default_env()
        .format(move |buf, record| TimeZoneFormat::new(buf, &timezone_fmt).write(record))
        .init();
    if let Some(cmd) = cli_opt.command {
        return run_subcommand(cmd).await;
    }
}