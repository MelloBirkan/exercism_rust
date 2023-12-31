mod reverse_string;
use reverse_string::reverse;

fn main() {
    let hello = "Hello, world!";
    let hello_reversed = reverse(hello);
    println!("{} reversed is {}", hello, hello_reversed);
}
