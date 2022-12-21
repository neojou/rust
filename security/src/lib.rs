pub struct Confidential {
    credit_card_number: String,
}

impl Confidential {
    pub fn new() -> Confidential {
	Confidential {
	    credit_card_number: String::from("1234567890123456"),
   	} 
    }

    pub fn sign(&self) -> String {
	let slice = self.credit_card_number[12..].to_string();
        slice
    }
}

