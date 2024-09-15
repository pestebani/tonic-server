mod error;

use crate::agenda::Agenda;
use crate::model::error::ModelError;

#[derive(Debug,Clone, PartialEq)]
pub struct AgendaModel {
    pub id: i64,
    pub name: String,
    pub email: String,
    pub phone: String,
}


impl AgendaModel {
    pub fn to_proto(self) -> Agenda {
        Agenda{
            id: self.id,
            name: self.name.clone(),
            email: self.email.clone(),
            phone: self.phone.clone(),

        }
    }

    pub fn from_proto(oa: Option<Agenda>) -> Result<Self, ModelError> {
        match oa {
            Some(ap) => Ok(AgendaModel{
                id: ap.id,
                name: ap.name.clone(),
                email: ap.email.clone(),
                phone: ap.phone.clone(),
            }),
            None => Err(ModelError::EmptyInput),
        }
    }
}
