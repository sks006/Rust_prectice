// enum Status{
//     Active,
//     Inactive,
//     Pending,
// }

// struct Loan{
//     id: u32,
//     amount: f64,
//     status: Status,
// }

// impl Loan{
//     fn new (id:u32,amount:f64,status:Status)->Loan{
//         Loan{
//             id,
//             amount,
//             status,
//         }
//     }
// }
// fn main(){
//     let l1=Loan::new(2,40000.0,Status::Pending);
//     match l1.status{
//         Status::Active => println!(" id {} .Loan is active",l1.id),
//         Status::Inactive => println!("id {} .Loan is inactive",l1.id),
//         Status::Pending => println!("id {} .Loan is pending",l1.id),
//     }
// }
//---------------------- 19 ------------------------
// enum Status{
//     Active,
//     Inactive,
//     Pending,
// }
// struct Loan{
//     id:u32,
//     amount:f64,
//     status:Status
// }

// impl Loan{
//     fn new(id:u32,amount:f64,status:Status)->Loan{
//         Loan{
//             id,amount,status
//         }
//     }
// }

// fn main(){
//     let l1=Loan::new(3,23422.3,Status::Active);
//     match l1.status{
//         Status::Pending=>println!("id is {} .Loan is pending",l1.id),
//         Status::Active=>println!("id is {} .Loan is Active",l1.id),
//         Status::Inactive=>println!("id is {} .Loan is Inactive",l1.id),
//     }
// }
//---------------------- 18 ----------------------------------------------
// enum Status{
//     Active,
//     Inactive,
//     Pending
// }
// struct Loan{
//     id:u32,
//     amount:f64,
//     status:Status
// }
// impl Loan{
//     fn new( id:u32,
//     amount:f64,
//     status:Status)->Self{
//         Self{
//             id,amount,status
//         }
//     }
// }

// fn main(){
//     let l1=Loan::new(2,2342.3,Status::Active);
//     match l1.status{
//         Status::Active=>println!("id is {} .Loan is Active",l1.id),
//         Status::Pending=>println!("id is {} .Loan is pending",l1.id),
//         Status::Inactive=>println!("id is {} .Loan is Inactive",l1.id),
//     }
// }

//----------------------------------- 17 --------------------------------
// enum Status{
//     pending,
//     active,
//     inactive
// }
// struct Loan{
//      id:u32,
//     amount:f64,
   
//     status:Status
// }
// impl Loan{
//     fn new(     id:u32, amount:f64,

//     status:Status)->Self{
//         Self{
//             id,amount,status
//         }
//     }
// }
// fn main(){
//     let l1=Loan::new(12,2333.3,Status::active);
//     match l1.status{
//         Status::active=>println!("id is {}.loan Active",l1.id),
//         Status::pending=>println!("id is {}.loan Pending",l1.id),
//         Status::inactive=>println!("id is {}.loan Inactive",l1.id),
//     }
// }
//----------------------------------- 16 --------------------------------
// enum Status{
// inactive,
// active,
// pending  
// }

// struct Loan{
//         id:u32,
//     amount:f64,
//     status:Status  
// }
// impl Loan{
//     fn new(id:u32,
//     amount:f64,
//     status:Status  )->Self{
//         Self{
//             id,
//             amount,
//             status
//         }
//     }
// }
// fn main(){
//     let l1=Loan::new(23,234234.3,Status::active);
//     match l1.status{
//                 Status::active=>println!("id is {}.loan Active",l1.id),
//         Status::pending=>println!("id is {}.loan Pending",l1.id),
//         Status::inactive=>println!("id is {}.loan Inactive",l1.id),
//     }
// }
//----------------------------------- 15 --------------------------------
// enum Status{
//     active,
//     inactive,
//     pending
// }
// struct Loan{
//     id:u32,
//     account:f64,
//     status:Status
// }

// impl Loan{
//     fn new(id:u32,account:f64,status:Status)->Self{
//         Self{
//             id,account,status
//         }
//     }
// }
// fn main(){
//     let l1=Loan::new(23,234234.3,Status::active);
//     match l1.status{
//         Status::active=>println!("id is {}.loan Active",l1.id),
//         Status::pending=>println!("id is {}.loan Pending",l1.id),
//         Status::inactive=>println!("id is {}.loan Inactive",l1.id),  
//     }
// }
//----------------------------------- 14 --------------------------------

// enum Status{
//     active,
//     inactive,
//     pending
// }
// struct Loan{
//     id:u32,
//     account:f64,
//     status:Status
// }

// impl Loan{
//     fn new(id:u32,account:f64,status:Status)->Self{
//         Self{
//             id,account,status
//         }
//     }
// }
// fn main(){
//     let l1=Loan::new(23,2323.3,Status::pending);
//     match l1.status{
//         Status::active=>println!("id is {}.loan Active",l1.id),
//         Status::pending=>println!("id is {}.loan Pending",l1.id),
//         Status::inactive=>println!("id is {}.loan Inactive",l1.id),  
//     }
// }
//----------------------------------- 13 --------------------------------
// enum Status{
//     active,
//     inactive, 
//     pending
// }
// struct Loan{
//     id:u32,
//     account:f64,
//     status:Status
// }
// impl Loan{
//     fn new(id:u32,account:f64,status:Status)->Self{
//         Self{
//             id,account,status
//         }
//     }
// }
// fn main(){
//     let l1=Loan::new(12,13212.2,Status::pending);
//     match l1.status{
//         Status::pending=>println!("id is {},loan pending",l1.id),
//         Status::active=>println!("id is {},loan active",l1.id),
//         Status::inactive=>println!("id is {},loan inactive",l1.id),
//     }
// }
//----------------------------------- 12 --------------------------------

// enum Status{
//     pending,
//     active,
//     inactive
// }

// struct Loan{
//     id:u32,
//     account:f64,
//     status:Status
// }
// impl Loan{
//     fn new(id:u32,account:f64,status:Status)->Self{
//         Self{
//             id,account,status
//         }
//     }
// }
// fn main(){
//     let l1=Loan::new(12,23443.3,Status::active);
//     match l1.status{
//             Status::pending=>println!("id is {},loan pending",l1.id),
//         Status::active=>println!("id is {},loan active",l1.id),
//         Status::inactive=>println!("id is {},loan inactive",l1.id),    
//     }
// }
//----------------------------------- 11 --------------------------------
// enum Status{
//     pending,
//     active,
//     inactive
// }

// struct Loan{
//     account:f32,
//     id:u32,
//     status:Status
// }

// impl Loan{
//     fn new(id:u32,account:f32,status:Status)->Self{
//         Self{
//             id,
//             account,
//             status
//         }
//     }
// }

// fn main(){
//     let l1=Loan::new(21,32532.4,Status::active);
//     match l1.status{
//         Status::pending=>println!("id is {},loan pending",l1.id),
//         Status::active=>println!("id is {},loan active",l1.id),
//         Status::inactive=>println!("id is {},loan inactive",l1.id),   
//     }
// }
//----------------------------------- 10 --------------------------------

// enum Status{
//     pending,
//     active,
//     inactive 
// }
// struct Loan{
//     id:u32,
//     account:f32,
//     status:Status
// }

// impl Loan{
//     fn new(id:u32,account:f32,status:Status)->Self{
//         Self{
//             id,account,status
//         }
//     }
// }

// fn main(){
//     let l1=Loan::new(12,23234.3,Status::active);
//     match l1.status{
//         Status::pending=>println!("id is {},loan pending",l1.id),
//         Status::active=>println!("id is {},loan active",l1.id),
//         Status::inactive=>println!("id is {},loan inactive",l1.id),  
//     }
// }
//----------------------------------- 9 --------------------------------

// enum Status{
//     pending,
//     active,
//     inactive
// }
// struct Loan{
//     id:u32,
//     account:f32,
//     status:Status
// }
// impl Loan{
//     fn new(id:u32, account:f32, status:Status
//     )->Self{
//         Self{
//             id,account,status
//         }
//     }
// }

// fn main(){
//     let l1=Loan::new(32,300.0,Status::pending);
//     match l1.status{
//               Status::pending=>println!("id is {},loan pending",l1.id),
//         Status::active=>println!("id is {},loan active",l1.id),
//         Status::inactive=>println!("id is {},loan inactive",l1.id),    
//     }
// }
//----------------------------------- 8 --------------------------------
// enum Status{
//     active,
//     pending,
//     inactive
// }

// struct Loan{
//     id:u32,
//     account:f32,
//     status:Status
// }

// impl Loan{
//     fn new(id:u32,account:f32,status:Status)->Self{
//         Self{
//             id,
//             account,
//             status
//         }
//     }
// }

// fn main(){
//     let l1=Loan::new(12,4000.4,Status::active);
//     match l1.status{
//         Status::pending=>println!("id is {},loan pending",l1.id),
//         Status::active=>println!("id is {},loan active",l1.id),
//         Status::inactive=>println!("id is {},loan inactive",l1.id),      
//     }
// }
//----------------------------------- 7 --------------------------------

// enum Status{
//     pending,
//     active,
//     inactive
// }
// struct Loan{
//     id:u32,
//     account:f32,
//     status:Status
// }
// impl Loan{
//     fn new(id:u32,account:f32,status:Status)->Self{
//         Self{
//             id,account,status
//         }
//     }
// }

// fn main(){
//     let l1=Loan::new(20,23423.4,Status::active);
//     match l1.status{
//             Status::pending=>println!("id is {},loan pending",l1.id),
//         Status::active=>println!("id is {},loan active",l1.id),
//         Status::inactive=>println!("id is {},loan inactive",l1.id),      
//     }
// }
//----------------------------------- 6 --------------------------------
// enum Status{
//     pending,
//     active,
//     inactive
// }
// struct Loan{
//     id:u32,
//     account:f32
//     status:Status
// }
// impl Loan{
//     fn new(id:u32,account:f32, status:Status)->Self{
//         Self{
//             id,account,status
//         }
//     }
// }
// fn main(){
//     let l1=Loan::new(12,22343.4,Status::pending);
//     match l1.status{
//                Status::pending=>println!("id is {},loan pending",l1.id),
//         Status::active=>println!("id is {},loan active",l1.id),
//         Status::inactive=>println!("id is {},loan inactive",l1.id),      
//     }
// }
//----------------------------------- 5 --------------------------------
// enum Status{
//     pending,
//     active,
//     inactive
// }

// struct Loan{
//     id:u32,
//     account:f32,
//     status:Status
// }

// impl Loan{
//     fn new(id:u32,account:f32,status:Status)->Self{
//         Self{
//             id,account,status
//         }
//     }
// }
// fn main(){
//     let l1=Loan::new(31,345534.4,Status::inactive);
//     match l1.status{
//         Status::pending=>println!("id is {},loan pending",l1.id),
//         Status::active=>println!("id is {},loan active",l1.id),
//         Status::inactive=>println!("id is {},loan inactive",l1.id),         
//     }
// }

//----------------------------------- 4 --------------------------------
// enum Status{
//     pending,
//     active,
//     inactive
// }
// struct Loan{
//     id:u32,
//     account:f32,
//     status:Status
// }
// impl Loan{
//     fn new(id:u32,account:f32,status:Status)->Self{
//         Self{
//             id,
//             account,
//             status
//         }
//     }
// }

// fn main(){
//     let l1=Loan::new(45,24342.3,Status::pending);
//     match l1.status{
//         Status::pending=>println!("id is {},loan pending",l1.id),
//         Status::active=>println!("id is {},loan active",l1.id),
//         Status::inactive=>println!("id is {},loan inactive",l1.id),    
//     }
// }
//----------------------------------- 3 --------------------------------
// enum Status{
//     pending,
//     active,
//     inactive
// }
// struct Loan{
//     id:u32,
//     account:f32,
//     status:Status
// }
// impl Loan{
//     fn new(id:u32,account:f32,status:Status)->Self{
//         Self{
//             id,account,status
//         }
//     }
// }
// fn main(){
//     let l1=Loan::new(23,24323.4,Status::inactive);
//     match l1.status{
//         Status::inactive=>println!("id is {}, loan inactive",l1.id),
//         Status::active=>println!("id is {}, loan inactive",l1.id),
//         Status::pending=>println!("id is {}, loan inactive",l1.id)
//     }
// }
//----------------------------------- 2 --------------------------------

// enum Status{
//     pending,
//     active,
//     inactive
// }
// struct Loan{
//     id:u32,
//     account:f32,
//     status:Status
// }
// impl Loan{
//     fn new(id:u32,account:f32,status:Status)->Self{
//         Self{
//             id,
//             account,
//             status
//         }
//     }
// }
// fn main(){
//     let l1=Loan::new(23,34532.4,Status::active);
//     match l1.status{
//         Status::inactive=>println!("id is {}, loan inactive",l1.id),
//         Status::active=>println!("id is {}, loan inactive",l1.id),
//         Status::pending=>println!("id is {}, loan inactive",l1.id)
//     }
// }
//----------------------------------- 1 --------------------------------

enum Status{
    pending,
    active,
    inactive
}
struct Loan{
    id:u32,
    account:f32,
    status:Status
}

impl Loan{
    fn new(    id:u32, account:f32,  status:Status)->Self{
        Self{
            id,account,status
        }
    }
}
fn main(){
    let l1=Loan::new(44,232.3,Status::inactive);
    match l1.status{
        Status::inactive=>println!("id is {}, loan inactive",l1.id),
        Status::active=>println!("id is {}, loan inactive",l1.id),
        Status::pending=>println!("id is {}, loan inactive",l1.id) 
    }
}