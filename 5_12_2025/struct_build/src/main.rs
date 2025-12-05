// struct Loan{
//     acc:u64
// }
// impl Loan{
//     fn new(a:u64)->Self{
//         Self{
//             acc:a
//         }
//     }
// }
// fn main(){
//     let l=Loan::new(4499);
//     println!("acc {}",l.acc)
// }

//------------------------------ 2 --------------------------------

// struct Loan{
//     acc:u64
// }
// impl Loan{
//     fn new(a:u64)->Self{
//         Self{
//             acc:a
//         }
//     }
// }
// fn main(){
//     let l=Loan::new(45678);
//     println!("acc {}",l.acc)
// }

//------------------------------ 3 --------------------------------
// enum Status{
//     pass,
//     reje
// }
// struct Loan{
//     acc:u64,
//     status:Status
// }

// impl Loan{
//     fn new(a:u64,s:Status)->Self{
//         Self{
//             acc:a,
//             status:s
//         }
//     }
// }

// fn main(){
//     let l=Loan::new(2342,Status::pass);
//     match l.status{
//         Status::pass=>println!("pass {}",l.acc),
//         Status::reje=>println!("reje {}",l.acc),
//     }
// }
//------------------------------ 4 --------------------------------

// enum Status{
//     pass,
//     reje
// }
// struct Loan{
//     acc:u64,
//     status:Status
// }
// impl Loan{
//     fn new(a:u64,s:Status)->Self{
//         Self{
//             acc:a,
//             status:s
//         }
//     } 
// }
// fn main(){
//     let l=Loan::new(234,Status::pass);
//     match l.status{
//         Status::pass=>println!("pass {}",l.acc),
//         Status::reje=>println!("reje {}",l.acc),
//     }
// }
//------------------------------ 5 --------------------------------
// struct Loan{
//     acc:u64
// }
// impl Loan{
//     fn new(a:u64)->Self{
//         Self{
//             acc:a
//         }
//     }
// }

// fn main(){
//     let l=Loan::new(456);
//     println!("acc {}",l.acc)
// }
//------------------------------ 6 --------------------------------

// struct Loan{
//     acc:u64
// }
// impl Loan{
//     fn new(a:u64)->Self{
//         Self{
//             acc:a
//         }
//     }
// }

// fn main(){
//     let l=Loan::new(234);
//     println!("acc {}",l.acc)
// }
//------------------------------ 7 --------------------------------

// struct Loan{
//     acc:u64
// }
// impl Loan{
//     fn new(a:u64)->Self{
//         Self{
//             acc:a
//         }
//     }
// }

// fn main(){
//     let l=Loan::new(4567);
//     println!{"acc {}",l.acc}
// }
//------------------------------ 8 --------------------------------

// struct Loan{
//     acc:u64
// }
// impl Loan{
//     fn new(a:u64)->Self{
//         Self{
//             acc:a
//         }
//     }
// }
// fn main(){
//     let l=Loan::new(4567);
//     println!("acc {}",l.acc)
// }
//------------------------------ 9 --------------------------------
// struct Loan {
//     acc:u64
// }
// impl Loan{
//     fn new(a:u64)->Self{
//         Self{
//             acc:a
//         }
//     }
// }
// fn main(){
//     let l=Loan::new(564);
//     println!("acc {}",l.acc)
// }
//------------------------------ 10 --------------------------------
// enum Status{
//     Pass,
//     Reje
// }
// struct Loan {
//     acc:u64,
//     status:Status
// }
// impl Loan{
//     fn new(a:u64,s:Status )->Self{
//         Self{
//             acc:a,
//             status:s
//         }
//     }
// }
// fn main(){
//     let l=Loan::new(564, Status::Pass );
//     println!("acc {}",l.acc);
//     match l.status{
//         Status::Pass=>println!("pass {}",l.acc),
//         Status::Reje=>println!("reje {}",l.acc),
//     }
// }
//------------------------------ 11 --------------------------------

// enum Status {
//     pass,
//     reje
// }
// struct Loan{
//     acc:u64,
//     status:Status
// }

// impl Loan {
//     fn new(a:u64,s:Status)->Self{
//         Self { acc: a, status: s }
//     }
// }

// fn main(){
//     let l=Loan::new(2342, Status::pass);
//     match l.status {
//         Status::pass=>println!("pass {}",l.acc),
//         Status::reje=>println!("reje {}",l.acc),
//     }
// }
//------------------------------ 12 --------------------------------

// enum Status {
//     pass,
//     reje
// }
// struct Loan{
//     acc:u32,
//     status:Status
// }
// impl Loan {
//     fn new(a:u32,s:Status)->Self{
//         Self { acc: a, status: s }
//     }
// }
// fn main(){
//     let l=Loan::new(243, Status::pass);
//     match l.status {
//         Status::pass=>println!("Pass {}",l.acc),
//         Status::reje=>println!("Reje {}",l.acc),
//     }
// }
//------------------------------ 13 --------------------------------

// enum Status {
//     pass,
//     reje
// }
// struct Loan{
//     acc:u32,
//     status:Status
// }
// impl Loan {
//     fn new(a:u32,s:Status)->Self{
//         Self{
//             acc:a,
//             status:s
//         }
//     }
// }
// fn main(){
//     let l=Loan::new(45678,Status::pass);
//     match l.status {
//                Status::pass=>println!("Pass {}",l.acc),
//         Status::reje=>println!("Reje {}",l.acc),
//     }
// }
//------------------------------ 14 --------------------------------

// enum Status {
//     pass,
//     reje
// }
// struct Loan{
//     acc:u32,
//     status:Status
// }

// impl Loan {
//     fn new(a:u32,s:Status)->Self{
//         Self { acc: a, status: s }
//     }
// }
// fn main(){
//     let l =Loan::new(21, Status::pass) ;
//     match l.status {
//                       Status::pass=>println!("Pass {}",l.acc),
//         Status::reje=>println!("Reje {}",l.acc),
//     }
// }
//------------------------------ 15 --------------------------------

// enum Status {
//     pass,
//     reje
// }
// struct Loan{
//     acc:u32,
//     status:Status
// }

// impl Loan {
//     fn new(a:u32,s:Status)->Self{
//         Self { acc: a, status: s }
//     }
// }
// fn main(){
//     let l=Loan::new(234, Status::pass);
//     match l.status {
//     Status::pass=>println!("Pass {}",l.acc),
//         Status::reje=>println!("Reje {}",l.acc), 
//     }
// }
//------------------------------ 16 --------------------------------
// enum Status {
//     pass,
//     reje
// }
// struct Loan{
//     acc:u32,
//     status:Status
// }
// impl Loan {
//     fn new(a:u32,s:Status)->Self{
//         Self { acc: a, status: s }
//     }
// }

// fn main(){
//     let l=Loan::new(123, Status::reje);
//     match l.status {
//     Status::pass=>println!("Pass {}",l.acc),
//         Status::reje=>println!("Reje {}",l.acc), 
//     }
// }
//------------------------------ 17 --------------------------------

// enum Status {
//     pass,
//     reje
// }
// struct Loan{
//     acc:u32,
//     status:Status
// }

// impl Loan {
//     fn new(a:u32,s:Status)->Self{
//         Self { acc: a, status: s }
//     }
// }
// fn main(){
//     let l=Loan::new(1232, Status::reje);
//     match l.status {
//         Status::pass=>println!("Pass {}",l.acc),
//         Status::reje=>println!("Reje {}",l.acc), 
//     }
// }
//------------------------------ 18 --------------------------------
// enum Status {
//     pass,
//     reje
// }
// struct Loan{
//     acc:u32,
//     status:Status
// }
// impl Loan {
//     fn new(a:u32,s:Status)->Self{
//         Self { acc: a, status: s }
//     }
// }
// fn main(){
//     let l=Loan::new(2343, Status::reje);
//         match l.status {
//         Status::pass=>println!("Pass {}",l.acc),
//         Status::reje=>println!("Reje {}",l.acc), 
//     }
// }
//------------------------------ 19 --------------------------------

// enum Status {
//     pass,
//     reje
// }
// struct Loan{
//     acc:u32,
//     status:Status
// }
// impl Loan {
//     fn new(a:u32,s:Status)->Self{
//         Self { acc: a, status: s }
//     }
// }
// fn main(){
//     let l=Loan::new(4533, Status::reje);
//     match l.status {
// Status::pass=>println!("Pass {}",l.acc),
// Status::reje=>println!("Reje {}",l.acc),
//     }
// }
//------------------------------ 20 --------------------------------

enum Status {
    pass,
    reje
}

struct Loan{
    acc:u32,
    status:Status
}
impl Loan {
    fn new(a:u32,s:Status)->Self{
        Self { acc: a, status: s }
    }
}

fn main(){
    let l=Loan::new(2342, Status::pass);
    match l.status {
 Status::pass=>println!("Pass {}",l.acc),
Status::reje=>println!("Reje {}",l.acc),       
    }
}