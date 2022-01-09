pub mod user_feature;

use crate::auth::User;
use crate::database::user_feature::EmploymentStatus;
use crate::database::user_feature::DBStatus;
use crate::database::user_feature::Gender;

pub struct Connection {
    status:DBStatus,
}

impl Connection {
    pub fn new(_url: String) -> Self {
        Connection{
            status: Connection::connect(Some(_url))
        }
    }

    pub fn get_data(&self, id:u16) -> Option<User> {
        return if self.status == DBStatus::Connected
        {
            let user = User {
                username: "test".to_string(),
                lastname: "test".to_string(),
                country: "test".to_string(),
                gender: Gender::Men,
                employment_status: EmploymentStatus::Employee,
                age: 18
            };
            Some(user)
        } else {
            None
        }
    }

    pub fn connect(url:Option<String>) -> DBStatus {
        return if Some(url).is_none() {
            DBStatus::NotConnected
        } else {
            DBStatus::Connected
        }
    }
}

