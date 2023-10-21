fn main() {
    let equation: &str = "7x = 35";
    println!("{}", algebra(equation));
}
//5x = 20
fn algebra(eq: &str) -> String {
    let mut coef: i32 = 0;
    let mut constant: i32 = 0;
    
    
    for i in 0..eq.len() {
        if &eq[i..i + 1] == "x" {
            if i == 0 {
                coef = 1;
            }
            else if &eq[i - 1..i] == " " {
                coef = 1;
            }
            else {
                coef = eq[i - 1..i].parse::<i32>().unwrap();
            }

        }
        if &eq[i..i + 1] == "=" {
            constant = eq[i + 2..].parse::<i32>().unwrap();
        }
    }
    let mut result: String = ("x = ").to_string();
    result += &(format!("{}", constant/coef)).to_string();
    
    return result;

  
}
