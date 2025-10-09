// // Unrecoverable errors
// fn main() {
//     println!("Hello, world!");

//     let arr=[1,2,3,4,5];
//     let el= arr[10];

//     println!("This is the end of the program, {}",el);
// }


// Recoverable errors

fn main(){
    let r=match divide(4,0){
        Ok(num)=> num,
        Err(err) => {
            println!("{err}");
            -1
        }
    };
    println!("r is: {}",r);
}

fn divide(x:i32,y:i32) -> Result<i32, String>{
    if y==0 {
        return Err(String::from("Please do not divide by 0"));
    }
    Ok(x/y)
}