mod urls;
mod utils;

use crate::urls::api_methods::{
    about_wallet,
};



fn main(){
    println!("Bot is started");
    let _ = about_wallet();
}