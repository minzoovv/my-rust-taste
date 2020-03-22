// 4-3
fn ownership_1 (){
    let s = String::from("hello");
    takes_ownership(s); // 소유권을 function argument s에게 넘겨줌
    
    // 여기서 s를 실행하려고 하면 에러가 남
    // println!("{}", s);

    let x = 5;
    makes_copy(x);
    // 여기선 copy이므로 에러가 나지 않음 
    println!("{}", x);
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
// fn ref_mut_ref(){
//     let mut s = String::from("hello"); // 가변 변수 생성, 

//     let s2 = &mut s;
//     change(&mut s); // 가변 참조를 생성
//      // 불가능. 특정 범위, 특정 데이터에 대해 2개 이상의 가변 참조를 만들 수 없음. 
// } 

// fn change(s: &mut String) {
//     s.push_str(" world"); // 가변 참조는 해당 변수에 대한 변경이 가능함. 
// } 

// // 4-7 reference - use both 
// fn ref_use_both(){
//     let mut s = String::from("this is test");
//     let a = &s;
//     let b = &s;
//     let c = &mut s; // 불변참조를 사용할 때 가변참조를 사용 할 수 없음.

//     println!("{} {}", a, b)
// }

// // 4-8 dead ref
// fn dead_ref(){
//     let reference_to_nothing = dangle();
// }

// fn dangle() -> String {
//     let s = String::from("hi");
    
//     s
// } // s will be dropped in this block end can return itself 

// 4-9 slice 

fn first_words_without_slice(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate(){
        if item == b' ' {
            return i; // 안의 depth에서 return을 하려면 return 예약어를 사용하는 듯?
        }
    }
    s.len()
}

fn first_words_with_slice(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate(){
        if item == b' '{
            return &s[0..i];
        }
    }
    &s[..]
}

// 이런 경우를 해결할 수 없다
fn main2(){
    let mut s = String::from("hi! that");
    let s1 = "this is literal";
    let word = first_words_without_slice(&s); // with slice는 이걸 해결 할 수 있다!
    let word2 = first_words_with_slice(&s1[..]);
    println!("{}", &s1[..]);

    // s.clear(); // 이미 s의 길이 값은 변했지만, word 값은 이전의 s 길이를 반영하고 있다.
    
}

fn slice(){
    let mut s = String::from("hello world");

    let hello = &s[..5];
    let o_world = &s[4..];
    // 그럼 이렇게는 될까?

    println!("{} {}", hello, o_world);
    let world = &s[6..];  
    let literal = "this is literal"; // datatype : &str
    // world.push_str(" new world");
    // println!("{}", world);
}

// 4-10 other slices

// fn type_id<T: std::any::Any>(_: &T) {
//     println!("{:?}", std::any::TypeId::of::<T>());
// }

fn other_slices(){

    let array = [4..5];
    let array_ref = &array[0..1];
    
}


fn main() {
    // ownership_1();
    // ownership_2();

    // ref_borrow();
    // ref_mut_ref();
    // ref_use_both();
    // dead_ref();
    
    // slice();
    main2();
    // other_slices();
}
