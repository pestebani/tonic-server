use crate::agenda::Agenda;


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

    pub fn from_proto(ap: Agenda) -> Self {
        AgendaModel{
            id: ap.id,
            name: ap.name.clone(),
            email: ap.email.clone(),
            phone: ap.phone.clone(),
        }
    }
}
