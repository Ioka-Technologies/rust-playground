use std::collections::HashMap;
use std::sync::Once;

use fake::faker::internet::raw::*;
use fake::faker::name::raw::*;
use fake::locales::*;
use fake::Fake;
use once_cell::sync::Lazy;

use crate::errors::base::{BaseError, Error};

static mut MEMBERS: Lazy<HashMap<i32, Member>> = Lazy::new(|| HashMap::new());

// INIT is used to seed the members only once
static INIT: Once = Once::new();

pub struct InputGet {
    id: i32,
}

pub struct Member {
    id: i32,
    name: String,
    email: String,
    is_deleted: bool,
    roles: Vec<String>,
}

pub enum MemberPatch {
    Name(String),
    Email(String),
    IsDeleted(bool),
}

fn seed(num_members: i32) {
    INIT.call_once(|| {
        // mutable statics require `unsafe`
        unsafe {
            MEMBERS.clear();

            for i in 0..num_members {
                MEMBERS.insert(
                    i,
                    Member::new(i, FirstName(EN).fake(), FreeEmail(EN).fake()),
                );
            }
        };
    })
}

impl Member {
    pub fn new(id: i32, name: String, email: String) -> Member {
        Member {
            id,
            name,
            email,
            is_deleted: false,
            roles: vec!["member".to_string()],
        }
    }

    pub fn deactivate(&'static mut self) -> Result<&'static mut Self, Error> {
        if self.id < 10 {
            return Err(Error::Internal(BaseError {
                message: "Deactivation failed".to_string(),
                reason: "Cannot deactivate member".to_string(),
            }));
        }
        Ok(self)
    }

    pub fn clear_roles(&'static mut self) -> Result<&'static mut Self, Error> {
        self.roles.clear();

        Ok(self)
    }

    pub fn update(
        &'static mut self,
        patches: Vec<MemberPatch>,
    ) -> Result<&'static mut Self, Error> {
        patches.iter().for_each(|patch| match patch {
            MemberPatch::Name(name) => self.name = name.to_string(),
            MemberPatch::Email(email) => self.email = email.to_string(),
            MemberPatch::IsDeleted(is_deleted) => self.is_deleted = *is_deleted,
        });

        Ok(self)
    }

    pub fn get(arg: InputGet) -> Result<&'static mut Self, Error> {
        seed(100);

        unsafe {
            if let Some(m) = MEMBERS.get_mut(&arg.id) {
                Ok(m)
            } else {
                Err(Error::NotFound(BaseError {
                    message: "Not found".to_string(),
                    reason: "Member not found".to_string(),
                }))
            }
        }
    }

    pub fn list() -> Vec<&'static Member> {
        unsafe { MEMBERS.values().collect() }
    }
}

pub struct Manager {}

impl Manager {
    pub fn deactivate_person(id: i32) -> Result<&'static Member, Error> {
        Ok(Member::get(InputGet { id })?
            .deactivate()?
            .clear_roles()?
            .update(vec![MemberPatch::IsDeleted(true)])?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_happy_day() {
        let result = Manager::deactivate_person(50);

        if let Ok(member) = result {
            assert_eq!(member.is_deleted, true);
            assert_eq!(member.roles.len(), 0);
        } else {
            panic!("Expected success");
        }
    }

    #[test]
    fn test_not_found() {
        if let Err(Error::NotFound(BaseError { message, reason })) = Manager::deactivate_person(101)
        {
            assert_eq!(message, "Not found");
            assert_eq!(reason, "Member not found");
        } else {
            panic!("Expected not found error");
        }
    }

    #[test]
    fn test_internal_error() {
        if let Err(Error::Internal(BaseError { message, reason })) = Manager::deactivate_person(5) {
            assert_eq!(message, "Deactivation failed");
            assert_eq!(reason, "Cannot deactivate member");
        } else {
            panic!("Expected internal error");
        }
    }
}
