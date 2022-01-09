pub enum Gender {
    Women,
    Men,
    Other,
}

pub enum EmploymentStatus {
    Employee,
    Employer,
}

#[derive(PartialEq)]
pub enum DBStatus {
    Connected,
    NotConnected,
}
