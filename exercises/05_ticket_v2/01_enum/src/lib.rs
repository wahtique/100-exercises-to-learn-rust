// TODO: use `Status` as type for `Ticket::status`
//   Adjust the signature and implementation of all other methods as necessary.

use std::str::FromStr;

#[derive(Debug, PartialEq, Clone)]
// `derive`s are recursive: it can only derive `PartialEq` if all fields also implement `PartialEq`.
// Same holds for `Debug`. Do what you must with `Status` to make this work.
struct Ticket {
    title: String,
    description: String,
    status: Status,
}

#[derive(Debug, PartialEq, Clone)]
enum Status {
    ToDo,
    InProgress,
    Done,
}

impl FromStr for Status {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "To-Do" => Ok(Status::ToDo),
            "In Progress" => Ok(Status::InProgress),
            "Done" => Ok(Status::Done),
            _ => Err(()),
        }
    }
}

impl Ticket {
    pub fn new(title: String, description: String, status: String) -> Ticket {
        if title.is_empty() {
            panic!("Title cannot be empty");
        }
        if title.len() > 50 {
            panic!("Title cannot be longer than 50 characters");
        }
        if description.is_empty() {
            panic!("Description cannot be empty");
        }
        if description.len() > 500 {
            panic!("Description cannot be longer than 500 characters");
        }

        let parsed_status = Status::from_str(&status)
            .expect("Only `To-Do`, `In Progress`, and `Done` statuses are allowed");

        Ticket {
            title,
            description,
            status: parsed_status,
        }
    }

    pub fn title(&self) -> &String {
        &self.title
    }

    pub fn description(&self) -> &String {
        &self.description
    }

    pub fn status(&self) -> &Status {
        &self.status
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::{valid_description, valid_title};

    #[test]
    fn test_partial_eq() {
        let title = valid_title();
        let description = valid_description();
        let ticket1 = Ticket {
            title: title.clone(),
            description: description.clone(),
            status: Status::ToDo,
        };
        let ticket2 = Ticket {
            title: title.clone(),
            description: description.clone(),
            status: Status::ToDo,
        };
        assert_eq!(ticket1, ticket2);
    }

    #[test]
    fn test_description_not_matching() {
        let title = valid_title();
        let status = Status::ToDo;
        let ticket1 = Ticket {
            title: title.clone(),
            description: "description".to_string(),
            status: status.clone(),
        };
        let ticket2 = Ticket {
            title: title.clone(),
            description: "description2".to_string(),
            status: status.clone(),
        };
        assert_ne!(ticket1, ticket2);
    }

    #[test]
    fn test_title_not_matching() {
        let description = valid_description();
        let status = Status::InProgress;
        let ticket1 = Ticket {
            title: "title".to_string(),
            description: description.clone(),
            status: status.clone(),
        };
        let ticket2 = Ticket {
            title: "title2".to_string(),
            description: description.clone(),
            status: status.clone(),
        };
        assert_ne!(ticket1, ticket2);
    }

    #[test]
    fn test_status_not_matching() {
        let title = valid_title();
        let description = valid_description();
        let ticket1 = Ticket {
            title: title.clone(),
            description: description.clone(),
            status: Status::InProgress,
        };
        let ticket2 = Ticket {
            title: title.clone(),
            description: description.clone(),
            status: Status::Done,
        };
        assert_ne!(ticket1, ticket2);
    }
}
