//sketches

use aes::Aes256;

use aes::cipher::{NewCipher, StreamCipher};
use aes::ctr::CtrStream;
use rand::prelude::*;

pub enum SourceTypes {
    TXT(String),
    NUM(i32),
}
trait Encryptions {
    fn encrypt(&self) -> String;
}
trait Decrypts {
    fn decrypt(&self) -> String;
}
trait Transfers {
    fn transfer(&self) -> String;
}


//----- Impls -------
pub struct Encryptor {
    pub sources : Vec<SourceTypes>, //this is the array of items to encrypt
    pub algo : String
}

impl Encryptor{
    pub fn NEW (sources : Vec<SourceTypes>, algo : Option<String>) -> Encryptor {
        Encryptor {
            sources : sources,
            algo : algo
        }
    }
}

impl Encryptions for Encryptor {
    fn encrypt (&self) -> String {
        return String::from("still nothing returns");
    }
}