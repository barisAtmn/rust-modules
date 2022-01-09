use crate::database::user_feature::EmploymentStatus;
use crate::database::user_feature::Gender;

pub struct User {
    pub username: String,
    pub lastname: String,
    pub country: String,
    pub gender: Gender,
    pub employment_status: EmploymentStatus,
    pub age:u8,
}

impl User {
    pub fn age_control (user:User) -> bool {
        if user.age >= 18 {
            true
        } else {
            false
        }
    }
}