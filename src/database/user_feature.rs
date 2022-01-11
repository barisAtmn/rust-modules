pub enum Gender {
    Woman,
    Man,
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
