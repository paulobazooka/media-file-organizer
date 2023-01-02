use std::time::{SystemTime};
use chrono::{DateTime, Local};

pub struct DateUtility {}

impl DateUtility {
    // Método para obter uma tupla (dia, mês, ano) a partir de uma struct SystemTime
    pub fn get_date(system_time: SystemTime) -> DateTime<Local> {
        system_time.clone().into()
    }
}