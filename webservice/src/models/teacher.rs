use crate::error::MyError;
use actix_web::web;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Teacher {
    pub id: i32, // serial
    pub name: String,
    pub picture_url: String,
    pub profile: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct CreateTeacher {
    pub name: String,
    pub picture_url: String,
    pub profile: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct UpdateTeacher {
    pub name: Option<String>,
    pub picture_url: Option<String>,
    pub profile: Option<String>,
}

impl TryFrom<web::Json<CreateTeacher>> for CreateTeacher {
    type Error = MyError;
    fn try_from(new_teacher: web::Json<CreateTeacher>) -> Result<Self, Self::Error> {
        Ok(CreateTeacher {
            name: new_teacher.name.clone(),
            picture_url: new_teacher.picture_url.clone(),
            profile: new_teacher.profile.clone(),
        })
    }
}

impl TryFrom<web::Json<UpdateTeacher>> for UpdateTeacher {
    type Error = MyError;

    fn try_from(new_teacher: web::Json<UpdateTeacher>) -> Result<Self, Self::Error> {
        Ok(UpdateTeacher {
            name: new_teacher.name.clone(),
            picture_url: new_teacher.picture_url.clone(),
            profile: new_teacher.profile.clone(),
        })
    }
}
