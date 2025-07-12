use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[derive(Clone)]
pub struct ConnectionConfig{
    ip: String,
    port: String,
    protocol: String,
    username: String,
    password: String,
    connect_name: String,
}

impl ConnectionConfig{
    pub fn new(ip: String, port: String, protocol: String, username: String, password: String,connect_name:String) -> ConnectionConfig{
        ConnectionConfig{ip, port, protocol, username, password, connect_name: connect_name }
    }
    pub fn get_ip(&self) -> &String{
        &self.ip
    }
    pub fn get_port(&self) -> &String{
        &self.port
    }
    pub fn get_protocol(&self) -> &String{
        &self.protocol
    }
    pub fn get_username(&self) -> &String{
        &self.username
    }
    pub fn get_password(&self) -> &String{
        &self.password
    }
    pub fn set_ip(&mut self, new_ip: String){
        self.ip = new_ip;
    }
    pub fn set_port(&mut self, new_port: String){
        self.port = new_port;
    }
    pub fn set_protocol(&mut self, new_protocol: String){
        self.protocol = new_protocol;
    }
    pub fn set_username(&mut self, new_username: String){
        self.username = new_username;
    }
    pub fn set_password(&mut self, new_password: String){
        self.password = new_password;
    }
    pub fn get_connect_name(&self) -> &String{
        &self.connect_name
    }
    pub fn copy(&mut self) -> ConnectionConfig{
        ConnectionConfig{
            ip:self.ip.clone(),
            port:self.port.clone(),
            protocol:self.protocol.clone(),
            username:self.username.clone(),
            password:self.password.clone(),
            connect_name:self.connect_name.clone(),
        }
    }
}
