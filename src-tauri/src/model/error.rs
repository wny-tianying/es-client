pub struct ConnectEsError {
    code: String,
    pub(crate) message: String,
}

impl ConnectEsError {
    pub fn new(code: String, message: String) -> Self {
        ConnectEsError{
            code,
            message,
        }
    }

    pub fn get_message(&self) -> String {
        self.message.clone()
    }

    pub fn get_code(&self) -> String {
        self.code.clone()
    }
}