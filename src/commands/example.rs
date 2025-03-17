use crate::error::Result;
use log::{info, debug};

pub fn execute(name: String, formal: bool) -> Result<()> {
    debug!("Executing example command with name: {}, formal: {}", name, formal);
    
    let greeting = if formal {
        format!("Good day, {}. How do you do?", name)
    } else {
        format!("Hey {}! What's up?", name)
    };
    
    info!("Greeting generated");
    println!("{}", greeting);
    
    Ok(())
}
