pub enum Status {
    Pending,
    Completed,
}

impl Status {
    pub fn as_str(&self) -> &str {
        match self {
            Status::Pending => "Pending",
            Status::Completed => "Completed",
        }
    }
}
pub struct Task {
    pub id: usize,
    pub title: String,
    pub status: Status,
}

impl Task {
    pub fn new(id: usize, title: String) -> Self {
        Self {
            id,
            title,
            status: Status::Pending, //every new task is pending
        }
    }

    pub fn complete(&mut self){
        self.status = Status::Completed;
    }

    pub fn is_completed(&self) -> bool {
        matches!(self.status, Status::Completed)
    }
}

