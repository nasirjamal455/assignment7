fn main(){
    crate::games::cricket::players();
}
pub mod games{
    pub mod cricket {
        pub fn players(){
            println!("there are eleven players in team");

        }
    }
}