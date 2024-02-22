use colored::*;

fn main() {
    if cfg!(target_os = "linux") {
        // everything in this block will be compiled
        // on all platforms, but only executed if
        // the target OS is linux
        let message = "Linux only messages incoming".green();
        println!("{}", message);
    } else {
        println!("The cfg! determined the system is {} Linux.", "not".red());
    }
    println!("{}","Hello, to all systems!".green());
    #[cfg(target_os = "windows")] {
        // this block will only be compiled if the
        // target OS is windows
        only_on_windows();
    }
    #[cfg(target_os = "macos")] {
        // this block will only be compiled if the
        // target OS is macos
        are_you_on_macos();
    }
    #[cfg(target_os = "linux")] {
        // this block will only be compiled if the
        // target OS is linux
        are_you_on_linux(); 
    }
}

#[cfg(target_os = "windows")]
fn only_on_windows() {
    let message = "Hello! BTW, this only appears for Windows systems";
    println!("{}", message.blue);
}
#[cfg(target_os = "macos")]
fn are_you_on_macos() {
    let message = "Hello! BTW, this only appears for MacOS systems";
    println!("{}", message).blue();
}
#[cfg(target_os = "linux")]
fn are_you_on_linux() {
    let message = "Hello! BTW, this only appears for Linux systems";
    println!("{}", message.blue());
}