

// fn divide(a:f32,b:f32)->Result<f32,String>{
//     if b==0.0{
//         Err("this is the end".to_string())
//     }else {
//         Ok(a/b)
//     }
// }
// fn main(){
//     let result=divide(21.0,3.0);
//     match result{
//         Ok(value)=>println!("value {}",value),
//         Err(error)=>println!("error {}",error)
//     }
// }
//----------------------- 2 ----------------------------------
// fn divide(a:u32,b:u32)->Result<u32,String>{
//     if b==0{
//         Err("this is the end".to_string())
//     }else {
//         Ok(a/b)
//     }
// }
// fn main(){
//     let s = divide(22, 4);
//     match s {
//         Ok(value)=>println!("value {}",value),
//         Err(error)=>println!("erroe {}",error)
//     }
// }
//----------------------- 3 ----------------------------------
// fn divide(a:f32,b:f32)->Result<f32,String>{
//     if b==0.0{
//         Err("this is Error".to_string())
//     }else {
//         Ok(a/b)
//     }
// }
// fn main(){
//     let s = divide(33.3, 4.2);
//     match s {
//         Ok(value)=>println!("Value {}",value),
//         Err(error)=>println!("error {}",error)
//     }
// }
//----------------------- 4 ----------------------------------
// fn divide(a:u32,b:u32)->Result<u32,String>{
//     if b==0{
//         Err("this is end".to_string())
//     }else {
//         Ok(a/b)
//     }
// }
// fn main(){
//     let x=divide(33, 4);
//     match x {
//          Ok(value)=>println!("Value {}",value),
//          Err(error)=>println!("error {}",error)
//     }
// }
//----------------------- 5 ----------------------------------

// use core::error;

// fn divide(a:u32,b:u32)->Result<u32,String>{
//     if b==0{
//         Err("this is end".to_string())
//     }else{
//         Ok(a/b)
//     }
// }

use core::error;
use std::{collections::btree_map::Values, result};

// fn main(){
//     let l=divide(33,0);
//     match l {
//      Ok(value)=>println!("value : {}",value),
//      Err(error)=>println!("error : {}", error)   
//     }
// }
//----------------------- 6 ----------------------------------
// fn divide(a:u32,b:u32)->Result<u32,String>{
//     if b==0{
//         Err("this is the end".to_string())
//     }else {
//         Ok(a/b)
//     }
// }
// fn main(){
//     let y=divide(32, 4);
//     match y {
//         Ok(value)=>println!("value {}",value),
//         Err(error)=>println!("error {}",error)
//     }
// }
//----------------------- 7 ----------------------------------
// fn divide(a:u32,b:u32)->Result<u32,String>{
//     if b==0{
//         Err("this is  the end".to_string())
//     }else {
//         Ok(a/b)
//     }
// }
// fn main(){
//     let l=divide(23, 23);
//      match l {
//          Ok(value)=>println!("value {}",value),
//          Err(error)=>println!("error {}",error)
//      }
// }
//----------------------- 8 ----------------------------------
// fn divide(s:f32,d:f32)->Result<f64,String>{
//     if d==0.0{
//         Err("this is the end".to_string())
//     }else {
//         Ok((s/d).into())
//     }
// }
// fn main(){
//     let l=divide(34.4, 2.0);
//     match l {
//          Ok(value)=>println!("value {}",value),
//          Err(error)=>println!("error {}",error)
//     }
// }
//----------------------- 9 ----------------------------------
// fn divide(a:f32,b:f32)->Result<f32,String>{
//     if b==0.0{
//         Err("this is the end".to_string())
//     }else {
//         Ok(a/b)
//     }
// }
// fn main(){
// let result=divide(333.3, 4.3);
// match result {
//      Ok(value)=>println!("value {}",value),
//      Err(error)=>println!("error {}",error)
// }
// }

//----------------------- 10 ----------------------------------
// fn divide(a:u32,b:u32)->Result<u32,String>{
// if b==0{
//     Err("this is the end".to_string())
// }else {
//     Ok(a/b)
// }
// }

// fn main(){
//     let result=divide(30, 41);
//     match result {
//          Ok(value)=>println!("value {}",value),
//          Err(error)=>println!("error {}",error)
//     }
// }
//----------------------- 11 ----------------------------------
// fn divide(a:u32,b:u32)->Result<u32,String>{
//     if b==0{
//         Err("error fuck up".to_string())
//     }else {
//         Ok(a/b)
//     }
// }
// fn main(){
// let result= divide(32, 2);
// match result {
//     Ok(value)=>println!("value {}",value),
//     Err(error)=>println!("error {}",error)
// }
// }
//----------------------- 12 ----------------------------------
// fn divide(a:f32,b:f32)->Result<f32,String>{
//     if b==0.0{
//         Err("this is the end".to_string())
//     }else {
//         Ok(a/b)
//     }
// }
// fn main(){
//     let k=divide(23.0,3.0);
//     match k {
//         Ok(value)=>println!("value {}",value),
//         Err(error)=>println!("error {}",error)
//     }
// }

//----------------------- 13 ----------------------------------

// fn divide(a:f32,b:f32)->Result<f32,String>{
//     if b==0.0{
//         Err("this is the end".to_string())
//     }else {
//         Ok(a/b)
//     }
// }
// fn main(){
//     let result=divide(3.0, 4.3);
//     match result {
//         Ok(values)=>println!("values {}",values),
//         Err(error)=>println!("error {}",error)
//     }
// }
//----------------------- 14 ----------------------------------
// fn divide(a:u32,b:u32)->Result<u32,String>{
//     if b==0{
//         Err("this is the end".to_string())
//     }else {
//         Ok(a/b)
//     }
// }

// fn main(){
//     let result=divide(34, 4);
//     match result {
//         Ok(values)=>println!("values {}",values),
//         Err(error)=>println!("error {}",error) 
//     }
// }
//----------------------- 15 ----------------------------------


fn divide(a:f32,b:f32)->Result<f32,String>{
    if b==0.0 {
        Err("this is the end".to_string())
    }else {
        Ok(a/b)
    }
}
fn main(){
    let x=divide(34.4, 2.3);
    match x {
             Ok(values)=>println!("values {}",values),
        Err(error)=>println!("error {}",error)   
    }
}