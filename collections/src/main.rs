fn main() {
    let mut v = vec!["a", "b", "c"];
    for i in &mut v{
        let a = String::from(*i);
        let aa = a + &"aa";
        println!("{}", a);
    }
    
}
