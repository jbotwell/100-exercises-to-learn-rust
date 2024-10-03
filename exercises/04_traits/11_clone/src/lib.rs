pub fn summary(ticket: Ticket) -> (Ticket, Summary) {
    (ticket.clone(), ticket.summary())
}

#[derive(Clone)]
pub struct Ticket {
    pub title: String,
    pub description: String,
    pub status: String,
}

impl Ticket {
    pub fn summary(&self) -> Summary {
        Summary {
            title: self.clone().title,
            status: self.clone().status,
        }
    }
}

pub struct Summary {
    pub title: String,
    pub status: String,
}
