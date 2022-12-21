use security::Confidential;

struct NotConfidential {
   out_card_no : String,
} 

fn main() {
    let a: i32 = 1000;
    let b: i8 = 1;
    //// compiler error because of the type difference: i8 to i32
    //// let c: i32 = a + b;
    let c: i32 = a + (b as i32);
    println!("{} + {} = {}", a, b, c);

    let u_value : u8 = 97;
    //// compiler error because of the type difference: u8 to char
    //// let a : char = u_value;
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

