use libloading::{Library, library_filename, Symbol};

fn main() {
    unsafe {
        let lib = Library::new(library_filename("plugin")).unwrap(); // Load the "hello_world" library
        let execute: Symbol<fn()> = lib.get(b"execute").unwrap(); // Get the function pointer

        execute() // Call the function
    }
}
