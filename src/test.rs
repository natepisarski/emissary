use emissary;

use serde::{Serialize, Deserialize};
use serde_derive::*;

#[derive(Serialize, Deserialize)]
enum Substance {
    Stimulant(String),
    Depressant(String),
    Antibiotic(String),
    Analgesic(String),
    Supplement(String),
    Other(String)
}

#[derive(Serialize, Deserialize)]
struct DosageSchedule {
    pub day_of_week: String,
    pub substance: Substance,
}

#[test]
pub fn test_emissary() {
    let schedule: DosageSchedule = DosageSchedule { day_of_week: "Thursday".to_string(), substance: Substance::Antibiotic("Penicillin".to_string())};
    let converted_schedule = emissary::create_emissary("schedule.thursday", schedule);
    let schedule_result = emissary::serialize_emissary(converted_schedule);

    
    println!("{}", schedule_result);
    assert!(true);
}