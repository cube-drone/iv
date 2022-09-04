
pub struct Identity{
    pub id: String,
    pub username: String,
    pub ip: String,
    pub fingerprint: String,
    pub country: String,
}

trait UserModel{
    fn identify(&self, username: &String, ip:&String, country:&String, fingerprint:&String) -> String;
    fn auth(&self, token:&String) -> Identity;
}

impl UserModel for Services {
    fn identify(&self, username: &String, ip: &String, country: &String, fingerprint: &String) -> String {
        String::from("Hi")
    }

    fn auth(&self, token:&String) -> Identity{
        Identity { 
            id: String::from("x9e8x8d7"), 
            username: String::from("barrymanilow"), 
            ip: String::from("192.168.1.1"), 
            fingerprint: String::from("aaaaa"), 
            country: String::from("CA") 
        }
    }
}