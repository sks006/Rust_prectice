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
enum Status{
    Pass,
    Reje
}
struct Loan {
    acc:u64,
    status:Status
}
impl Loan{
    fn new(a:u64,s:Status )->Self{
        Self{
            acc:a,
            status:s
        }
    }
}
fn main(){
    let l=Loan::new(564, Status::Pass );
    println!("acc {}",l.acc);
    match l.status{
        Status::Pass=>println!("pass {}",l.acc),
        Status::Reje=>println!("reje {}",l.acc),
    }
}