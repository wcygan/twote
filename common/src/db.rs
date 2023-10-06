pub enum MongoDB {
    Profiles,
}

impl MongoDB {
    pub fn name(&self) -> &str {
        match self {
            Self::Profiles => "profiles-db",
        }
    }

    pub fn uri(&self) -> String {
        match self {
            Self::Profiles => format!("mongodb://{}:27017", self.name()),
        }
    }
}

pub enum MongoCollection {
    Profiles,
}

impl MongoCollection {
    pub fn name(&self) -> &str {
        match self {
            Self::Profiles => "profiles",
        }
    }
}
