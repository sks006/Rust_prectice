// struct Loan{
//     acc:u32,
// }

// impl Loan{
//     fn new(acc:u32)->Loan{
//         Loan{acc}
//     }
// }

// fn main() {
//     let my_loan = Loan::new(5000);
//     println!("Loan account number: {}", my_loan.acc);
// }
// -------------------------- 14 --------------------------

// enum Status{
//     Active,
//     Inactive,
//     Pending,
// }
// struct Loan{
//     acc:u32,
//     status:Status,
// }
// impl Loan{
//     fn new(acc:u32,status:Status)->Loan{
//         Loan{acc,status}
//     }
// }
// fn main(){
//     let my_loan= Loan::new(3000,Status::Active);
//     match my_loan.status{
//         Status::Active=>println!("Loan is active"),
//         Status::Inactive=>println!("Loan is inactive"),
//         Status::Pending=>println!("Loan is pending"),
//     }
// }
// -------------------------- 13 --------------------------
// enum Status{
//     Active,
//     Inactive,
//     Pending,
// }
// struct Loan{
//     acc:u32,
//     status:Status,
// }
// impl Loan{
//     fn new(acc:u32,status:Status)->Loan{
//         Loan{acc,status}
//     }
// }
// fn main(){
//     let my_loan=Loan::new(4000,Status::Pending);
//     match my_loan.status{
//         Status::Active=>println!("Loan is active"),
//         Status::Inactive=>println!("Loan is inactive"),
//         Status::Pending=>println!("Loan is pending"),
//     }
// }
// -------------------------- 12 --------------------------
// enum Status{
//     Active,
//     Inactive,
//     Pending,
// }
// struct Loan{
//     acc:u32,
//     status:Status
// }
// impl Loan{
//     fn new (acc:u32,status:Status)->Self{
//         Self{acc,status}
//     }
// }
// fn main(){
//     let my_loan=Loan::new(6000,Status::Inactive);
//     match my_loan.status{
//         Status::Active=>println!("Loan is active"),
//         Status::Inactive=>println!("Loan is inactive"),
//         Status::Pending=>println!("Loan is pending"),
//     }
// }
// -------------------------- 11 --------------------------
// enum Status{
//     Active,
//     Inactive,
//     Pending,
// }

// struct Loan{
//     acc:u32,
//     status:Status,
// }
// impl Loan{
//     fn new(a:u32,s:Status)->Self{
//         Self{acc:a,status:s}
//     }
// }
// fn main(){
//     let my_loan=Loan::new(8000,Status::Active);
//     match my_loan.status{
//         Status::Active=>println!("Loan is active"),
//         Status::Inactive=>println!("Loan is inactive"),
//         Status::Pending=>println!("Loan is pending"),
//     }
// }
// -------------------------- 10 --------------------------

// enum Status{
//     active,
//     inactive,
//     pending,
// }
// struct Loan{
//     acc:u32,
//     status:Status,
// }
// impl Loan{
//     fn new(a:u32,s:Status)->Self{
//         Self{acc:a,status:s}
//     }
// }
// fn main(){
//     let my_loan=Loan::new(9000,Status::pending);
//     match my_loan.status{
//         Status::active=>println!("Loan is active"),
//         Status::inactive=>println!("Loan is inactive"),
//         Status::pending=>println!("Loan is pending"),
//     }
// }
// -------------------------- 9 --------------------------
// enum Status{
//     active,
//     inactive,
//     pending,
// }
// struct Loan{
//     acc:u32,
//     status:Status,
// }
// impl Loan{
//     fn new(a:u32,s:Status)->Self{
//         Self{acc:a,status:s}
//     }
// }
// fn main(){
//     let my_loan=Loan::new(2323,Status::inactive);
//     match my_loan.status{
//         Status::active=>println!("Loan is active"),
//         Status::inactive=>println!("Loan is inactive"),
//         Status::pending=>println!("Loan is pending"),
//     }
// }
// -------------------------- 8 --------------------------
// enum Status{
//     active,
//     inactive,
//     pending,
// }
// struct Loan{
//     acc:u32,
//     status:Status,
// }
// impl Loan{
//     fn new(a:u32,s:Status)->Self{
//         Self{acc:a,status:s}
//     }
// }

// fn main(){
//     let my_loan=Loan::new(12342,Status::active);
//     match my_loan.status{
//         Status::active=>println!("Loan is active"),
//         Status::inactive=>println!("Loan is inactive"),
//         Status::pending=>println!("Loan is pending"),
//     }
// }
// -------------------------- 7 --------------------------
// enum Status{
//     Active,
//     Inactive,
//     Pending,
// }
// struct Loan{
//     acc:u32,
//     status:Status,
// }

// impl Loan{
//     fn new(a:u32,s:Status)->Self{
//         Self{acc:a,status:s}
//     }
// }

// fn main(){
//     let my_loan=Loan::new(2222,Status::Pending);
//     match my_loan.status{
//         Status::Active=>println!("Loan is active"),
//         Status::Inactive=>println!("Loan is inactive"),
//         Status::Pending=>println!("Loan is pending"),}
// }
// -------------------------- 6 --------------------------

// enum Status{
//     Active,
//     Inactive,
//     Pending,
// }
// struct Loan{
//     acc:u32,
//     status:Status,
// }
// impl Loan{
//     fn new(a:u32, s:Status)->Self{
//         Self{acc:a,status:s}
//     }
// }
// fn main(){
//     let my_loan=Loan::new(2322,Status::Active);
//     match my_loan.status{
//         Status::Active=>println!("Loan is active"),
//         Status::Inactive=>println!("Loan is inactive"),
//         Status::Pending=>println!("Loan is pending"),
//     }
// }
// -------------------------- 5 --------------------------

// enum Status{
//     Active,
//     Inactive,
//     Pending,
// }
// struct Loan{
//     acc:u32,
//     status:Status,
// }
// impl Loan{
//     fn new(a:u32,s:Status)->Self{
//         Self{acc:a,status:s}
//     }
// }

// fn main(){
//     let my_loan=Loan::new(2323,Status::Inactive);
//     match my_loan.status{
//         Status::Active=>println!("Loan is active"),
//         Status::Inactive=>println!("Loan is inactive"),
//         Status::Pending=>println!("Loan is pending"),
//     }
// }
// -------------------------- 4 --------------------------
// enum Status{
//     Active,
//     Inactive,
//     Pending,
// }
// struct Loan{
//     acc:u32,
//     status:Status,
// }
// impl Loan{
//     fn new(a:u32,s:Status)->Self{
//         Self{acc:a,status:s}
//     }
// }

// fn main(){
//     let my_loan=Loan::new(3939,Status::Pending);
//     match my_loan.status{
//         Status::Active=>println!("Loan is active"),
//         Status::Inactive=>println!("Loan is inactive"),
//         Status::Pending=>println!("Loan is pending"),   

//     }
// }
// -------------------------- 3 --------------------------
// enum Status{
//     active,
//     inactive,
//     pending,
// }
// struct Loan{
//     acc:u32,
//     status:Status,
// }
// impl Loan{
//     fn new(a:u32,s:Status)->Self{
//         Self{acc:a,status:s}
//     }
// }

// fn main(){
//     let my_loan=Loan::new(2020,Status::pending);
//     match my_loan.status{
//         Status::active=>println!("Loan is active"),
//         Status::inactive=>println!("Loan is inactive"),
//         Status::pending=>println!("Loan is pending"),
//     }
// }
// -------------------------- 2 --------------------------
// enum Status{
//     active,
//     inactive,
//     pending,
// }
// struct Loan{
//     acc:u32,
//     status:Status,
// }
// impl Loan{
//     fn new(a:u32,s:Status)->Self{
//         Self{acc:a,status:s}
//     }
// }
// fn main(){
//     let my_loan=Loan::new(1010,Status::inactive);
//     match my_loan.status{
//         Status::active=>println!("Loan is active"),
//         Status::inactive=>println!("Loan is inactive"),
//         Status::pending=>println!("Loan is pending"),
//     }
// }
// -------------------------- 1 --------------------------
enum Status{
    active,
    inactive,
    pending,
}
struct Loan{
    acc:u32,
    status:Status,
}
impl Loan{
    fn new(a:u32,s:Status)->Self{
        Self{acc:a,status:s}
    }
}
fn main(){
    let my_loan=Loan::new(1001,Status::active);
    match my_loan.status{
        Status::active=>println!("Loan is active"),
        Status::inactive=>println!("Loan is inactive"),
        Status::pending=>println!("Loan is pending"),
    }
}