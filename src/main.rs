// //-------------class assignment------------
// #[derive(Debug)]
// pub struct Post
// {
//     pub linkedin: String,
// }

// pub trait Summary
// {
//     fn summarize(&self) -> String
//     {
//         format!("This is the summary of this post")
//     }
// }
// pub trait new_notification
// {
//     fn alert(&self) -> String
//     {
//         format!("This is the summary of this post")
//     }
// }

// impl Summary for Post
// {
//     fn summarize(&self) -> String
//     {
//         format!("bbvvcc")
//     }
// }
// impl new_notification for Post
// {
//     fn alert(&self) -> String
//     {
//         format!("bbvvcc")
//     }
// }
// fn notify(item: impl Summary+new_notification)
// {
//     println!("{} {}", item.summarize(), item.alert());
// }
// fn main()
// {
//    let notification = Post
//    {
//        linkedin: String::from("Morning"),
//    };

//    notify(notification);
// }
//*********************not completed failed */
//------------make a struct of post
//------------new notification trait method returns hey you have got a new notification
//------------summarize trait method returns this is the summary of this post
//-------------notify() function should accept a struct only if struct has both of these traits
//-----implemented

//---------13.1

// fn main()
// {
//------------closure-----------
// let x = 1;
// let f = |x| x+1;
// println!("{}", f(2));

// let x = || println!("hello");
// println!("{:?}", x());

//--------------to solve---------------
// let mut y;
// let y = 1;
// let z = 2;
// let x = |y,z| {y = y+1; z= z+1};
//-----------------------------------

// let mut x = 1;
// let mut f = &mut || {x = x + 1; println!("x = {}",x)};
// f();
// f();
// f();

// }
// fn hello<T: Fn(u32)->u32> (x:T) -> u32
// {
//         x(2)
// }
// fn main()
// {
//     let mut x = |y| y+1;
//     println!("{}",hello(x));
// }
