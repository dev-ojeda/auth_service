use auth_service::Credentials;
use auth_service::user::user::User;
pub fn main()
{
    let cred: Credentials = Credentials{
        username: String::from("test"),
        password: String::from("test")
    };

    let objUsuario = auth_service::login(cred);

    println!("USER: {}.",objUsuario.user());
    println!("PASS: {}.",objUsuario.pass());
}