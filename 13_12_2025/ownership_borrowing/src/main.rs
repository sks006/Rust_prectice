// fn borrow(a:String)->u32{
//     let x:u32=a.parse().expect("this is the end");
//     let y:u32=1000;
//     let z:u32=x-y;
//     return z
// }
// fn main(){
//     let l=String::from("10002");
//     let y=borrow(l);
//     println!("value {}",y)
// }
// ---------------------------- 14 ---------------------------------
// fn borrow(a:String)->u32{
//     let x:u32=a.parse().expect("this is the end");
//     let y:u32=1000;
//     let z:u32=x+y;
//     return z
// }
// fn main(){
//     let l=String::from("200000");
//     let y=borrow(l);
//     println!("value {}",y)
// }
// ---------------------------- 14 ---------------------------------
// fn borrow(a:String)->u32{
//     let x:u32=a.parse().expect("this is the end");
//     let y:u32=1000;
//     let z:u32=x+y;
//     return z
// }
// fn main(){
//     let l=String::from("40000");
//     let y=borrow(l);
//     println!("value {}",y)
// }
// ---------------------------- 13 ---------------------------------
// fn borrow(a:String)->u32{
//     let x:u32=a.parse().expect("this is the end");
//     let y:u32=1000;
//     let z:u32=x-y;
//     return z
// }

// fn main(){
//     let l=String::from("100000");
//     let y=borrow(l);
//     println!("value {}",y)
// }
// ---------------------------- 12 ---------------------------------
// fn borrow(a:String)->u32{
//     let x:u32=a.parse().expect("this is the end");
//     let y:u32=4000;
//     let z:u32=x-y;
//     return z
// }
// fn main(){
//     let l=String::from("50000");
//     let y=borrow(l);
//     println!("value {}",y)
// }
// ---------------------------- 11 ---------------------------------
// fn borrow(a:String)->u32{
//     let x:u32=a.parse().expect("this is the end");
//     let y:u32=3000;
//     let z:u32=x-y;
//     return z
// }
// fn main(){
//     let l=String::from("60000");
//     let y=borrow(l);
//     println!("value {}",y)
// }
// ---------------------------- 10 ---------------------------------
// fn borrow(a:String)->u32{
//     let x:u32=a.parse().expect("this is the end");
//     let y:u32=8000;
//     let z:u32=x+y;
//     return z
// }
// fn main(){
//     let l=String::from("70000");
//     let y=borrow(l);
//     println!("value {}",y)
// }
// ---------------------------- 9 ---------------------------------
// fn borrow(a:String)->u32{
//     let x:u32=a.parse().expect("this is the end");
//     let y:u32=3000;
//     let z:u32=x-y;
//     println!("borrow {}",z);
//     return z
// }
// fn main(){
//     let l=String::from("76000");
//     let y=borrow(l);
//     println!("value {}",y)
// }
// ---------------------------- 8 ---------------------------------
// fn borrow(a:String)->u32{
//     let x:u32=a.parse().expect("this is the end");
//     let y:u32=7000;
//     let z:u32=x+y;
//     return z
// }
// fn main(){
//     let l=String::from("300000");
//     let y=borrow(l);
//     println!("value {}",y)
// }
// ---------------------------- 7 ---------------------------------
// fn borrow(a:String)->u32{
// let x:u32=a.parse().expect("this is the end");
// let y:u32=10000;
// let z:u32=x*y;
// return z
// }
// fn main(){
//         let l=String::from("300000");
//     let y=borrow(l);
//     println!("value {}",y)
// }
// ---------------------------- 6 ---------------------------------
// fn borrow(a:String)->u32{
//     let x:u32=a.parse().expect("this is the end");
//     let y:u32=2000;
//     let z:u32=x-y;
//     return x-y
// }
// fn main(){
//     let l=String::from("300000");
//     let y=borrow(l);
//     println!("value {}",y)
// }
// ---------------------------- 5 ---------------------------------
// fn borrow(a:String)->u32{
//     let x:u32=a.parse().expect("this is the end");
//     let y:u32=300;
//     let z:u32=x-y;
//     return z
// }
// fn main(){
//     let l=String::from("400000");
//     let y=borrow(l);
//     println!("value {}",y);
// }
// ---------------------------- 4 ---------------------------------
// fn borrow(a:String)->u32{
//     let x:u32=a.parse().expect("this is the end");
//     let y:u32=400;
//     let z:u32=x-y;
//     return z
// }
// fn main(){
//     let l=String::from("4000000");
//     let y=borrow(l);
//     println!("value {}",y)
// }
// ---------------------------- 3 ---------------------------------
// fn borrow(a:String)->u32{
//     let x:u32=a.parse().expect("this is the end");
//     let y:u32=600;
//     let z:u32=x+y;
//     return z
// }
// fn main(){
//     let l=String::from("6000000");
//     let y=borrow(l);
//     println!("value {}",y)
// }
// ---------------------------- 2 ---------------------------------
// fn borrow(a:String)->u32{
// let x:u32= a.parse().expect("this is the end");
// let y:u32=4000;
// let z:u32=x-y;
// return z
// }

// fn main(){
//     let l=String::from("5500000");
//     let y=borrow(l);
//     println!("value {}",y)
// }
// ---------------------------- 1 ---------------------------------
// fn borrow(a:String)->u32{
//     let x:u32=a.parse().expect("this is the end");
//     let y:u32=3000;
//     let z:u32=x-y;
//     return z
// }
// fn main(){
//     let l=String::from("7000000");
//     let y=borrow(l);
//     println!("value {}",y)
// }
// ---------------------------- 15 ---------------------------------
// fn borrow(a:&String){
//     let x:u32=a.parse().expect("this is the end");
//     let y:u32=3000;
//     let z:u32=x-y;
//     println!("bor value {}",z)
// }

// fn main(){
//     let l=String::from("500000");
//     borrow(&l);
//     println!("value {}",l)
// }
// ---------------------------- 14 ---------------------------------
// fn borrow(a:&String){
//     let x:u32=a.parse().expect("this is the end");
//     let y:u32=999;
//     let z:u32=x-y;
//     println!("borrow {}",z)
// }
// fn main(){
//     let l=String::from("899999");
//     borrow(&l);
//     println!("value {}",l)
// }
// ---------------------------- 13 ---------------------------------
// fn borrow(a:&String){
//     let x:u32=a.parse().expect("this is the end");
//     let y:u32=80;
//     let z:u32=x-y;
//     println!("bor {}",z)
// }
// fn main(){
//     let l=String::from("9000000");
//     borrow(&l);
//     println!("value {}",l)
// }
// ---------------------------- 12 ---------------------------------
// fn borrow(a:&String){
//     let x:u32=a.parse().expect("this is the end");
//     let y:u32=899;
//     let z:u32=x-y;
//     println!("value {}",z)
// }
// fn main(){
//     let x=String::from("5000");
//     borrow(&x);
//     println!("value {}",x)
// }
// ---------------------------- 11 ---------------------------------
// fn borrow(a:&String){
//     let x:u32=a.parse().expect("this is the end");
//     let y:u32=400;
//     let z:u32=x-y;
//     println!("value {}",z)
// }
// fn main(){
//     let x=String::from("8000");
//     borrow(&x);
//     println!("value {}",x)
// }
// ---------------------------- 10 ---------------------------------
// fn borrow(a:&String){
//     let x:u32=a.parse().expect("this is the end");
//     let y:u32=500;
//     let z:u32=x-y;
//     println!("value {}",z)
// }
// fn main(){
//     let x=String::from("9000");
//     borrow(&x);
//     println!("value {}",x)
// }
// ---------------------------- 9 ---------------------------------
// fn borrow(a:&String){
//     let x:u32=a.parse().expect("this is the end");
//     let y:u32=600;
//     let z:u32=x*y;
//     println!("value {}",z)
// }
// fn main(){
//     let x=String::from("10000");
//     borrow(&x);
//     println!("value {}",x)
// }
// ---------------------------- 8 ---------------------------------
// fn borrow(a:&String){
//     let x:u32=a.parse().expect("this is the end");
//     let y:u32=5000;
//     let z:u32=x+y;
//     println!("borrow {}",z)
// }
// fn main(){
//     let x=String::from("20000");
//     borrow(&x);
//     println!("value {}",x)
// }
// ---------------------------- 7 ---------------------------------
// fn borrow(a:&String){
//     let x:u32=a.parse().expect("this is the end");
//     let y:u32=800;
//     let z:u32=x+y;
//     println!("borrow {}",z)
// }
// fn main(){
//     let x=String::from("40000");
//     borrow(&x);
//     println!("value {}",x)
// }
// ---------------------------- 6 ---------------------------------
// fn borrow(a:&String){
//     let x:u32=a.parse().expect("this is the end");
//     let y:u32=500;
//     let z:u32=x+y;
//     println!("value {}",z)
// }
// fn main(){
//     let x=String::from("500000");
//     borrow(&x);
//     println!("value {}",x)
// }
// ---------------------------- 5 ---------------------------------
// fn borrow(a:&String){
//     let x:u32=a.parse().expect("this is the end");
//     let y:u32=700;
//     let z:u32=x+y;
//     println!("value {}",z)
// }
// fn main(){
//     let x=String::from("5000000");
//     borrow(&x);
//     println!("value {}",x)
// }
// ---------------------------- 4 ---------------------------------
// fn borrow(a:&String){
//     let x:u32=a.parse().expect("this is the end");
//     let y:u32=400;
//     let z:u32=x*y;
//     println!("value {}",z)
// }
// fn main(){
//     let x=String::from("600000");
//     borrow(&x);
//     println!("value {}",x)
// }
// ---------------------------- 3 ---------------------------------
// fn borrow(a:&String){
//     let x:u32=a.parse().expect("this is the end");
//     let y:u32=400;
//     let z:u32=x*y;
//     println!("value {}",z)
// }
// fn main(){
//     let x=String::from("700000");
//     borrow(&x);
//     println!("value {}",x)
// }
// ---------------------------- 3 ---------------------------------
// fn borrow(a:&String){
//     let l:u32=a.parse().expect("this is the end");
//     let y:u32=80;
//     let z:u32=l-y;
//     println!("value {}",z)
// }
// fn main(){
//     let x=String::from("800000");
//     borrow(&x);
//     println!("value {}",x)
// }
// ---------------------------- 2 ---------------------------------
// fn borrow(a:&String){
//     let x:u32=a.parse().expect("this is the end");
//     let y:u32=699;
//     let z:u32=x-y;
//     println!("value {}",z)
// }
// fn main(){
//     let x=String::from("200000");
//     borrow(&x);
//     println!("value {}",x)
// }
// ---------------------------- 1 ---------------------------------
fn borrow(a:&String){
    let x:u32=a.parse().expect("this is the end");
    let y:u32=200;
    let z:u32=x-y;
    println!("value {}",z)
}
fn main(){
    let x=String::from("100000");
    borrow(&x);
    println!("value {}",x) 
}