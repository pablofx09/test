fn main() {
    
    let equation: &str = "4=3+45x";
    //println!("{}", algebra(equation));
    
    let z: &str = arrange(equation).as_str();
    println!("{}", z);
    
    
    
}
//5x = 20
fn algebra(eq: &str) -> String {
    let mut coef: i32 = 0;
    let mut constant: i32 = 0;
    let mut left: i32 = 0;
    let equals = finder(eq);
    
    
    
    for i in 0..eq.len() {
        if &eq[i..i + 1] == "x" {
            if i == 0 {
                coef = 1;
            }
            else if &eq[i - 1..i] == "-" {
                coef = -1;
            }
            else {
                coef = eq[..i].parse::<i32>().unwrap();
            }
            if &eq[i + 1..i + 2] != "=" {
                left = eq[i + 1..equals].parse::<i32>().unwrap();
            }

        }
        if &eq[i..i + 1] == "=" {
            constant = eq[i + 1..].parse::<i32>().unwrap();
        }
    }
    let mut result: String = ("x = ").to_string();
    constant -= left;
    result += &(format!("{}", constant/coef)).to_string();
    
    
    return result;

  
}

fn finder(eq: &str) -> usize {
    for i in 0..eq.len() {
        if &eq[i..i + 1] == "=" {
            return i;
        }
    }
    return 0 as usize;
}
fn arrange(eq: &str) -> String {
    let mut temp = String::new();
    let _equals: usize = finder(eq);
    for i in 0..eq.len() {
    
        if &eq[i..i + 1] == "x" {
            if i == 0 {
                temp += "x";
            }
            else if &eq[i - 1..i] == "+" || &eq[i - 1..i] == "=" {
                temp += "x";
            }
            else if &eq[i - 1..i] == "-" {
                temp += "-x";
            }
            else {
                //start where the x is, loop through backwards until you get a +, -, =, or at the start of the string
                //3+45x=9
                let mut start: i32 = i as i32;
                for j in (0..i).rev() {
                    if j == 0 {
                        start = 0;
                    }
                    else if &eq[j..j + 1] == "+" || &eq[j..j + 1] == "-" || &eq[j..j + 1] == "="  {
                        break;
                    }
                    else {
                        start -= 1;
                    }
                }
                temp += &eq[start as usize..i];
                temp += "x";
                

            }

        }
    
    }
    println!("{}", temp);
    //let fin: &str = temp.as_str();
    return temp;
}


