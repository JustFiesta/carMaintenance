use argon2rs::argon2i_simple;
use hex::encode;
use rand::Rng;

use crate::menage::Menage;
use crate::vehicles::vehicles::Vehicle;

pub struct User{
    pub user_id: u32,
    pub name: String,
    pub login: String,
    pub password_hash: String,
    pub email: String,
    pub phone: String,
    pub role: UserRole,
    pub salt: String,
}

pub enum UserRole{
    User,
    Admin,
}

impl User{
    //searches in database and logs in
    pub fn log_in(&self) {
        //TODO: database implementation

        todo!()
    }
    //exits to main menu
    fn log_out(&self) {
        todo!()
    }
    //sends new user data to database and logs user
    fn sign_in(&self) {
        todo!()
        //Database operations and queries
        //...
        //in main use after this .log_in()
    }
    //vell it hashes the password xd
    pub fn hash_password(&mut self, password: &str) {
        let mut rng = rand::thread_rng();
        let salt: String = (0..32)
            .map(|_| rng.gen_range(33..127) as u8 as char)
            .collect();
        self.salt = salt.clone(); //puts random salt to user
        let hash_result = argon2i_simple(password, &salt); // hash table
        self.password_hash = encode(&hash_result); // converts hash table to string
    }

    fn change_def_vehicle(&mut self, vehicle: &Vehicle) {
        todo!()
    }
}



impl Menage for User {
    fn add(&mut self) {
        todo!()
    }

    fn remove(&mut self) {
        todo!()
    }

    fn edit(&mut self) {
        todo!()
    }
}