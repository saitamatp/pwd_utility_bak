#[allow(non_snake_case)]
mod StartServer;
use StartServer::{ConnStruct,DecyptedValues,RecieveResponse,SendResponse,RecData,SendRes,Data};
mod encrypt;
#[allow(non_snake_case)]
use encrypt::encrypt::{encrypt_string,decryt_string,error};   



pub fn decrypt_conn(a:String)->String{
    
    let contents:ConnStruct=serde_json::from_str(&a).expect("Unable to read input Json");
    let master:String=contents.Password;
    let mut a:Vec<DecyptedValues>=Vec::new();

    for val in contents.values.iter(){
        a.push(DecyptedValues { acc:val.Acc.clone(), pwd: decryt_string(&val.Data, master.clone()).expect("cannot decrypt") })
    }

    serde_json::to_string(&a).expect("Connot convert to Json String")
}

pub fn encrypt_conn(a:String)->String{
    let mut contents:RecieveResponse=serde_json::from_str(&a).expect("Unable to read input Json");
    let master:String=contents.Password.clone();
    let mut a:Vec<DecyptedValues>=Vec::new();

    for val in contents.values.iter(){
        a.push(DecyptedValues { acc:val.Acc.clone(), pwd: decryt_string(&val.Data, master.clone()).expect("cannot decrypt") })
    }
    let mut checker =true;

    for val in a.iter(){
        if val.pwd=="Invalid Password"{
            print!("Invalid password");
            checker=false;
        }
    }

if checker==false{
    let f=Data{
                    Acc:"Invalid Passowrd".to_string(),
                    Data:vec![73,110,118,97,108,105,100,32,80,97,115,115,119,111,114,100]
    };
    let mut j:Vec<Data>=Vec::new();
    j.push(f);
    let c=SendResponse{
                isValid:false,
                values: j
    };
 serde_json::to_string(&c).expect("Connot convert to Json String")

}else if checker==true {
    /*Write Encrypt code  */
    let encrypt_pwd=encrypt_string(contents.pwd.clone(), contents.Password);
    let acc:String=contents.acc;

    contents.values.push(Data { Acc: acc, Data: encrypt_pwd.unwrap() });

   let k:SendResponse=SendResponse{
    isValid:true,
    values:contents.values
   };

    serde_json::to_string(&k).expect("Connot convert to Json String")
}else{
    print!("Invalid password");
    "Inavlid".to_string()
}
}


pub fn encypt_first(a:String)->String{
     let contents:RecData=serde_json::from_str(&a.clone()).expect("Unable to read input Json");
     //let encyptedPwd=encrypt_string(contents.pwd, contents.Password).expect("Unable to encypt from encrypt");
     let acc=contents.acc;
     let k:SendRes;
    let  content:RecData=serde_json::from_str(&a).expect("Unable to read input Json");


    let p=content.Password.len();

     if p>=32{
        let encypted_pwd=encrypt_string(contents.pwd, contents.Password).expect("Unable to encypt from encrypt");
        let l:Data=Data{
        Acc:acc,
        Data:encypted_pwd.clone()
     };
      k=SendRes{ isValid:true,values:l };
      serde_json::to_string(&k).expect("Connot convert to Json String")
     }else {
         let l:Data=Data{
            Acc:acc,
            Data:error()
         };
    k=SendRes{ isValid:false,values:l };
       serde_json::to_string(&k).expect("Connot convert to Json String")
     }

     
}