#[derive(Debug, PartialEq, Clone)]
pub enum Role {
    CEO,
    Manager,
    Worker,
}

#[derive(Debug)]
pub struct WorkEnvironment {
    pub grade: Link,
}
impl From<&str> for Role {
    fn from(rl: &str) -> Role {
        match rl {
            "Manager" => Role::Manager,
            "CEO" => Role::CEO,
            _ => Role::Worker,
        }
    }
}
pub type Link = Option<Box<Worker>>; // Complete type alias

#[derive(Debug)]
pub struct Worker {
    pub role: Role,
    pub name: String,
    pub next: Link,
}

impl WorkEnvironment {
    pub fn new() -> Self {
        Self { grade: None }
    }

    pub fn add_worker(&mut self, name: &str, role: &str) {
        let old = self.grade.take();
        let new = Box::new(Worker {
            role: Role::from(role),
            name: name.to_string(),
            next: old,
        });
        self.grade = Some(new);
    }

    pub fn remove_worker(&mut self) -> Option<String> {
        let head = self.grade.take()?;
        self.grade = head.next;
        Some(head.name)
    }

    pub fn last_worker(&self) -> Option<(String, Role)> {
        match &self.grade {
            Some(v) => Some((v.name.clone(), v.role.clone())),
            None => None,
        }
    }
}
