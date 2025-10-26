use std::process::Command;
use std::str::from_utf8;
use std::io::Result;

pub fn api_ping() -> Result<()>{

    let output = Command::new("curl")
        .arg("https://api.binance.com/api/v3/time")
        .output()?;

    if output.status.success(){
        let response = from_utf8(&output.stdout).unwrap();
        println!("Ping on binance {}", response);
    }else{
        let error = from_utf8(&output.stderr).unwrap();
        println!("Ошибка {}", error);
    };

    Ok(())
}