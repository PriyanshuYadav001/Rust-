fn main(){
   let mut x: i8=-5;
   let y: u32=123;
   let z: f32=1246.846;

   println!("x:{}, y:{}, z:{}",x,y,z);

   for i in 0..1000{
        x=x+100;
   }
   println!("x:{}",x);
}

fn main(){
    let is_male =false;
    let mut is_above_18=true;

    is_above_18=false;

    if is_male{
        println!("You are a male.");
    }
    else{
        println!("You are not a male.");
    }

    if is_male && is_above_18{
        println!("You are a legal male.");
    }
}


fn main(){
    let greeting = String::from("Hello World!");  // can change space at runtime
    println!("{}",greeting);

    let char1=greeting.chars().nth(0);

    match char1{
        Some(c) => println!("{}",c),
        None => println!("No character at given index."),
    }

}

fn main(){
    let is_even = true;

    if(is_even){
        println!("Even number.");
    }
    else if !is_even{
        println!("Odd number.");
    }
}

fn main(){
    for i in 1..11{
        print!("{} ",i);
    }
    println!();
}

iterate over arrays, maps, strings

fn main(){
    let sentence = String::from("My name is Priyanshu.");
    let first_word = get_first_word(sentence);
    println!("First word: {}",first_word);
}

fn get_first_word(sentence: String)-> String{
    let mut ans = String::from("");
    for char in sentence.chars(){
        ans.push_str(char.to_string().as_str());
        if char==' '{
            break;
        }
    }
    return ans;
}

