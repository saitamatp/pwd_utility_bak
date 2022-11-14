use serde::{Deserialize, Serialize};
use validator::{Validate};

#[derive(Debug,Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct Data{
    pub Acc:String,
    pub Data:Vec<u8>
}

/*Decrypt */
#[derive(Debug,Serialize, Deserialize,Validate)]
#[allow(non_snake_case)]
pub struct ConnStruct{
    #[validate(length(min = 32))]
    pub Password:String,
    pub values:Vec<Data>
}

#[derive(Debug,Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct DecyptedValues{
    pub acc:String,
    pub pwd:String
}
/*Decrypt */

/*Encrypt */
#[derive(Debug,Serialize, Deserialize,Validate)]
#[allow(non_snake_case)]
pub struct RecieveResponse{
    #[validate(length(min = 32))]
    pub Password:String,
    pub values:Vec<Data>,
    pub acc:String,
    pub pwd:String
}

#[derive(Debug,Serialize, Deserialize)]
#[allow(non_snake_case)]
//Encrypted Response to react
pub struct SendResponse{
    pub isValid:bool,
    pub values: Vec<Data>
}
/*Encrypt */

/*New Encrypt */
#[derive(Debug,Serialize, Deserialize,Validate)]
#[allow(non_snake_case)]
pub struct RecData{
    #[validate(length(min = 32))]
    pub Password:String,
    pub acc:String,
    pub pwd:String
}
#[derive(Debug,Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct SendRes{
    pub isValid:bool,
    pub values: Data
}
/*New Encrypt */