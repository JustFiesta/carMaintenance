use crate::menage::Menage;

struct Maintenance {
    vehicle_id: u32,
    part: String,
    replacement_date: chrono::NaiveDate,
    cost: u16,
    place: String,
    next_maintenance: chrono::NaiveDate,
}

impl Menage for Maintenance {
    fn add(&mut self) {
        todo!()
    }

    fn remove(&mut self) {
        todo!()
    }

    fn edit(&mut self) {
        todo!()
    }
}