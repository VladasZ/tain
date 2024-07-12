use std::{borrow::Cow, collections::HashMap};

use testcontainers::{
    core::{ContainerPort, WaitFor},
    Image,
};

const NAME: &str = "postgres";
const TAG: &str = "16-alpine";

// pub type PostgresArc = Arc<Container<'static, Postgres>>;

#[derive(Debug)]
pub struct Postgres {
    ports:    Vec<ContainerPort>,
    env_vars: HashMap<String, String>,
}

impl Postgres {
    // pub fn sokolikcik2<T>(setup: impl FnOnce(T) -> T) -> PostgresArc {
    //     static POSTGRES: OnceLock<Weak<Container<T>>> = OnceLock::new();
    //
    //     todo!()
    // }

    // pub fn sokolikcik(setup: impl FnOnce() -> Container<'static, Postgres>) ->
    // PostgresArc {     static POSTGRES: OnceLock<Weak<Container<Postgres>>> =
    // OnceLock::new();     static LOCK: Mutex<()> = Mutex::new(());
    //
    //     let _init = LOCK.lock().unwrap();
    //
    //     if let Some(weak) = POSTGRES.get() {
    //         return weak.upgrade().unwrap();
    //     }
    //
    //     let container: Arc<Container<Postgres>> = Arc::new(setup());
    //     let weak = Arc::downgrade(&container);
    //
    //     POSTGRES.set(weak).unwrap();
    //
    //     container
    // }
}

impl Postgres {
    pub fn db(mut self, db_name: impl ToString) -> Self {
        self.env_vars.insert("POSTGRES_DB".to_owned(), db_name.to_string());
        self
    }

    pub fn user(mut self, user: impl ToString) -> Self {
        self.env_vars.insert("POSTGRES_USER".to_owned(), user.to_string());
        self
    }

    pub fn password(mut self, password: impl ToString) -> Self {
        self.env_vars.insert("POSTGRES_PASSWORD".to_owned(), password.to_string());
        self
    }

    pub fn data(mut self, password: impl ToString) -> Self {
        self.env_vars.insert("PGDATA".to_owned(), password.to_string());
        self
    }
}

impl Default for Postgres {
    fn default() -> Self {
        let mut env_vars = HashMap::new();
        env_vars.insert("POSTGRES_DB".to_owned(), "postgres".to_owned());
        env_vars.insert("POSTGRES_USER".to_owned(), "postgres".to_owned());
        env_vars.insert("POSTGRES_PASSWORD".to_owned(), "postgres".to_owned());

        Self {
            ports: vec![5432.into()],
            env_vars,
        }
    }
}

impl Image for Postgres {
    fn name(&self) -> &str {
        NAME
    }

    fn tag(&self) -> &str {
        TAG
    }

    fn ready_conditions(&self) -> Vec<WaitFor> {
        vec![WaitFor::message_on_stderr(
            "database system is ready to accept connections",
        )]
    }

    fn env_vars(&self) -> impl IntoIterator<Item = (impl Into<Cow<'_, str>>, impl Into<Cow<'_, str>>)> {
        self.env_vars.iter()
    }

    fn expose_ports(&self) -> &[ContainerPort] {
        &self.ports
    }
}
