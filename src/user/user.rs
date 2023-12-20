
pub struct User {
    user: String,
    pass: String,
    users: Option<Box<User>>    
}

impl User {
    pub fn new(user: String, pass: String, users: Option<Box<User>>) -> Self {
        User {
            user,
            pass,
            users
        }
    }

    pub fn with_users(users: Option<Box<User>>) -> Self {
        User { user: String::new(), pass: String::new(), users }
    }

    pub fn default() -> Self {
        User { user: String::new(), pass: String::new(), users: None }
    }

    pub fn user(&self) -> &str {
        &self.user
    }

    pub fn set_user(&mut self, user: String) {
        self.user = user;
    }
    
    pub fn pass(&self) -> &str {
        &self.pass
    }
    
    pub fn set_pass(&mut self, pass: String) {
        self.pass = pass;
    }
    
    pub fn users(&self) -> Option<&Box<User>> {
        self.users.as_ref()
    }
    
    pub fn set_users(&mut self, users: Option<Box<User>>) {
        self.users = users;
    }

    pub fn list_user(users: User) -> User {
        return  users;
    }

}

