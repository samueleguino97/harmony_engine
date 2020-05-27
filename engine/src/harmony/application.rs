#[macro_export]
macro_rules! create_application {
    ($application_type:ident) => {{
        fn main {
            let application = $application_type::new();
            application.run()
        }
    }};
}


pub trait Application{
    fn new()->Self;
    fn run(&self){
        loop{}
    }
    fn create_application(){
        
    }
    
    
}