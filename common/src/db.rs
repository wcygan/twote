pub enum MongoDB {
    Profiles,
    Tweets,
}

impl MongoDB {
    pub fn name(&self) -> &str {
        match self {
            Self::Profiles => "profiles-db",
            Self::Tweets => "tweets-db",
        }
    }

    pub fn uri(&self) -> String {
        match self {
            Self::Profiles => format!("mongodb://{}:27017", self.name()),
            Self::Tweets => format!("mongodb://{}:27018", self.name()),
        }
    }
}

pub enum MongoCollection {
    Profiles,
    Tweets,
}

impl MongoCollection {
    pub fn name(&self) -> &str {
        match self {
            Self::Profiles => "profiles",
            Self::Tweets =>  "tweets",
        }
    }
}
