pub mod user;
use user::user::User;

pub struct Credentials{
    pub username: String,
    pub password: String,
}

pub enum Status {
    Connected,
    Interrupted,
}

fn connect_db ()-> Status {
    return Status::Connected;
}

pub fn login(creds: Credentials) -> User{
    let mut lsuarios = User::default();
    lsuarios.set_user(creds.username);
    lsuarios.set_pass(creds.password);
    return User::list_user(lsuarios);
}

fn logout(){
    
}

fn get_user(){

}

pub fn authenticate(creds: Credentials) {
    if let Status::Connected = connect_db() {
        login(creds);
    }
}

