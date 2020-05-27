extern crate harmony_engine;

use harmony_engine::harmony::application::Application;

struct Sandbox {}
impl Application for Sandbox {
    fn new()->Sandbox {
        Sandbox{}    
    }
}

harmony_engine::create_application!(Sandbox);

fn main() {
    let application:Sandbox = Sandbox::new(); 
    application.run();

}
