use crate::common::AppSysConfig;
use bean_factory::FactoryData;
use chrono::FixedOffset;
use std::sync::Arc;

pub struct AppShareData {
    pub sys_config: Arc<AppSysConfig>,

    pub factory_data: FactoryData,
    pub timezone_offset: Arc<FixedOffset>,
}
