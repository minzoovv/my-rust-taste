// 4-3
fn ownership_1 (){
    let s = String::from("hello");
    takes_ownership(s); // 소유권을 function argument s에게 넘겨줌
    
    // 여기서 s를 실행하려고 하면 에러가 남
    // println!("{}", s);

    let x = 5;
    makes_copy(x);
    // 여기선 copy이므로 에러가 나지 않음 
    // println!("{}", x);
}

fn takes_ownership(some_str: String){
    println!("{}", some_str); // 소유권은 여기에 머물고
} // 여기에서 drop됨. 
 
fn makes_copy(some_int: i32){
    println!("{}", some_int);
}

// 4-4 with return 
fn ownership_2(){
    let s1 = gives_ownership();

    let s2 = takes_and_gives_back(s1);

    // 에러가 안나게쬬
    println!("{}", s2);
}

fn gives_ownership() -> String{
    let s = String::from("hello");
    s
}

fn takes_and_gives_back(s: String) -> String{
    s
} 

// 4-5 reference - borrow 
fn ref_borrow(){
    let s1 =  String::from("hi");

    let len = calculate_len(&s1); // 참조, 즉 대여를 해준 것이므로 소유권은 s1에 머물러 있음
    println!("{} {}", s1, len); // 에러 안남. 
}

fn calculate_len(s: &String) -> usize {
    s.len()
} // 참조를 가져왔기 때문에 s1이 drop되지 않음.

/*
fn change(s: &String) {
    s.push_str(" world");
} // 이 함수는 실행하면 에러가 남. 참조 또한 immutable이므로 
*/

// 4-6 reference - mut ref 
fn ref_mut_ref(){
    let mut s = String::from("hello"); // 가변 변수 생성, 
    let s1 = &mut s; // 가변 참조를 생생성
    change(s1); 

   // let s2 = &mut s; // 불가능. 특정 범위, 특정 데이터에 대해 2개 이상의 가변 참조를 만들 수 없음. 
} 

fn change(s: &mut String) {
    s.push_str(" world"); // 가변 참조는 해당 변수에 대한 변경이 가능함. 
} 

// 4-7 reference - use both 
fn ref_use_both(){
    let mut s = String::from("this is test");
    let a = &s;
    let b = &s;
    // let c = &mut s; // 불변참조를 사용할 때 가변참조를 사용 할 수 없음.

    println!("{} {}", a, b)
}

// // 4-8 dead ref
// fn dead_ref(){
//     let reference_to_nothing = dangle();
// }

// fn dangle() -> String {
//     let s = String::from("hi");
    
//     &s
// } // s will be dropped in this block end can return itself 

fn main() {
    ownership_1();
    ownership_2();
    ref_borrow();
    ref_mut_ref();
    ref_use_both();
}
