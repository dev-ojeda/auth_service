use auth_service::Credentials;

pub fn main()
{
    let cred: Credentials = Credentials{
        username: String::from("test"),
        password: String::from("test")
    };

    let obj_usuario = auth_service::login(cred);

    println!("USER: {}.",obj_usuario.user());
    println!("PASS: {}.",obj_usuario.pass());
}