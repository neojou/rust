use security::Confidential;

struct NotConfidential {
   out_card_no : String,
} 

fn main() {
    let u_value : u8 = 97;
    let a : char = u_value as char;
    println!("a = {}", a);

    let cf = Confidential::new();

    //// compiler error since credit_card_number is a private data
    //// println!("card number: {}", cf.credit_card_number);
    println!("sign: {}", Confidential::sign(&cf));

    let out_cf : NotConfidential = unsafe {
	std::mem::transmute(cf)
    };
    println!("credit card number: {}", out_cf.out_card_no);
}

