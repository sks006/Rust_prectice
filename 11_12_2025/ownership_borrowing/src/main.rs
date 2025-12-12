// fn borrow(a:String)->u32{
//     let x:u32=a.parse().expect("this is the end");
//     let y:u32=232134;
//     let z:u32=x-y;
//     return z
// }
// fn main (){
//     let l=String::from("3000000");
//     let y=borrow(l);
//     println!("value {}",y)
// }
//--------------------- 9 -------------------------

// fn borrow(a:String)->u32{
//     let x:u32=a.parse().expect("this is the end");
//     let y:u32=23233;
//     let z:u32=x-y;
//     return z
// }
// fn main (){
//     let l=String::from("3000000");
//     let y=borrow(l);
//     println!("value {}",y)
// }
//--------------------- 8 -------------------------
fn borrow(a:String)->u32{
    let x:u32=a.parse().expect("this is the end");
    let y:u32=23233;
    let z:u32=x-y;
    return z
}
fn main(){
    let l=String::from("40000000");
    let y=borrow(l);
    println!("value {}",y)
    
}