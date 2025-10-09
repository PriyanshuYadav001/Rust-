fn main() {
    let mut s=String::from("Hello, world!"); // main is the owner of s
    s.push_str("from here.");
    {
        let _x="Hey from X"; // x scope starts
    } // x scope ends
}
