use irsc::server;

pub mod config;

pub struct RustyBot{
        
    //connections to irc servers
    connections: Vec<server::Server>
}

impl RustyBot {

    pub fn new() -> RustyBot {

        RustyBot{
            connections: Vec::new()    
        }
    }

    pub fn connect(){
        
    }
}
