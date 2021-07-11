use flowy_database::{
    query_dsl::*,
    schema::{user_table, user_table::dsl},
    DBConnection,
    ExpressionMethods,
};
use flowy_infra::kv::KVStore;
use lazy_static::lazy_static;
use std::sync::RwLock;

use crate::{
    entities::{SignInParams, SignUpParams, UserDetail, UserId},
    errors::UserError,
    services::user_session::{
        database::UserDB,
        user_server::{UserServer, *},
    },
    sql_tables::User,
};

pub struct UserSessionConfig {
    root_dir: String,
}

impl UserSessionConfig {
    pub fn new(root_dir: &str) -> Self {
        Self {
            root_dir: root_dir.to_owned(),
        }
    }
}

pub struct UserSession {
    database: UserDB,
    config: UserSessionConfig,
    server: Box<dyn UserServer + Send + Sync>,
}

impl UserSession {
    pub fn new<R>(config: UserSessionConfig, register: R) -> Self
    where
        R: 'static + UserServer + Send + Sync,
    {
        let db = UserDB::new(&config.root_dir);
        Self {
            database: db,
            config,
            server: Box::new(register),
        }
    }

    pub async fn sign_in(&self, params: SignInParams) -> Result<User, UserError> {
        let user = self.server.sign_in(params)?;
        let _ = set_current_user_id(Some(user.id.clone()))?;
        self.save_user(user)
    }

    pub async fn sign_up(&self, params: SignUpParams) -> Result<User, UserError> {
        let user = self.server.sign_up(params)?;
        let _ = set_current_user_id(Some(user.id.clone()))?;
        self.save_user(user)
    }

    pub async fn sign_out(&self) -> Result<(), UserError> {
        let user_id = self.current_user_id()?;
        let conn = self.get_db_connection()?;
        let affected =
            diesel::delete(dsl::user_table.filter(dsl::id.eq(&user_id))).execute(&*conn)?;

        match self.server.sign_out(&user_id) {
            Ok(_) => {},
            Err(_) => {},
        }
        let _ = self.database.close_user_db()?;
        let _ = set_current_user_id(None)?;
        // debug_assert_eq!(affected, 1);

        Ok(())
    }

    pub async fn current_user_detail(&self) -> Result<UserDetail, UserError> {
        let user_id = self.current_user_id()?;
        let conn = self.get_db_connection()?;

        let user = dsl::user_table
            .filter(user_table::id.eq(&user_id))
            .first::<User>(&*conn)?;

        match self.server.get_user_info(&user_id) {
            Ok(_user_detail) => {
                // TODO: post latest user_detail to upper layer
            },
            Err(_e) => {
                // log::debug!("Get user details failed. {:?}", e);
            },
        }

        Ok(UserDetail::from(user))
    }

    pub fn get_db_connection(&self) -> Result<DBConnection, UserError> {
        match get_current_user_id()? {
            None => Err(UserError::Auth("User is not login yet".to_owned())),
            Some(user_id) => self.database.get_connection(&user_id),
        }
    }
}

impl UserSession {
    fn save_user(&self, user: User) -> Result<User, UserError> {
        let conn = self.get_db_connection()?;
        let result = diesel::insert_into(user_table::table)
            .values(user.clone())
            .execute(&*conn)?;

        Ok(user)
    }

    fn current_user_id(&self) -> Result<String, UserError> {
        match KVStore::get_str(USER_ID_DISK_CACHE_KEY) {
            None => Err(UserError::Auth("No login user found".to_owned())),
            Some(user_id) => Ok(user_id),
        }
    }
}

const USER_ID_DISK_CACHE_KEY: &str = "user_id";
lazy_static! {
    pub static ref CURRENT_USER_ID: RwLock<Option<String>> = RwLock::new(None);
}
pub(crate) fn get_current_user_id() -> Result<Option<String>, UserError> {
    let read_guard = CURRENT_USER_ID
        .read()
        .map_err(|e| UserError::Auth(format!("Read current user id failed. {:?}", e)))?;

    let mut user_id = (*read_guard).clone();
    // explicitly drop the read_guard in case of dead lock
    drop(read_guard);

    if user_id.is_none() {
        user_id = KVStore::get_str(USER_ID_DISK_CACHE_KEY);
        *(CURRENT_USER_ID.write().unwrap()) = user_id.clone();
    }

    Ok(user_id)
}

pub(crate) fn set_current_user_id(user_id: Option<String>) -> Result<(), UserError> {
    KVStore::set_str(
        USER_ID_DISK_CACHE_KEY,
        user_id.clone().unwrap_or("".to_owned()),
    );

    let mut current_user_id = CURRENT_USER_ID
        .write()
        .map_err(|e| UserError::Auth(format!("Write current user id failed. {:?}", e)))?;
    *current_user_id = user_id;
    Ok(())
}