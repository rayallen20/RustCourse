use my_project::my_module::say_hello;
use my_project::my_module::sub_module::say_hello as sub_say_hello;

fn main() {
    say_hello();
    sub_say_hello();
}
