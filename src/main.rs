extern crate base64;
extern crate libc;
extern crate objc_foundation;
#[macro_use]
extern crate objc;
extern crate objc_id;
#[link(name = "AppKit", kind = "framework")]
extern "C" {}

use base64::decode;
use objc::runtime::{Class, Object};
use objc_foundation::{INSArray, INSString};
use objc_foundation::{NSArray, NSString};
use objc_id::Id;

fn main() {
    cli();
}

fn try_decode(input_str: &str) -> Option<String> {
    decode(input_str)
        .ok()
        .and_then(|vu8| String::from_utf8(vu8).ok())
}

fn decode_full(b64_str: &str) -> Option<String> {
    let mut res = try_decode(b64_str)?;
    while let Some(again) = try_decode(&res) {
        res = again;
    }
    Some(res)
}

fn cli() {
    match get_input() {
        Some(b64_str) => match decode_full(&b64_str) {
            Some(s) => {
                println!("{}", s);
                if add_to_clipboard(&s).is_err() {
                    eprintln!("Sorry! Could not copy to clipboard!");
                } else {
                    println!("Copied to clipboard for your convience!");
                }
            }
            None => println!("Error: Can't decode that!"),
        },
        None => eprintln!("Please provide a base64-encoded string!"),
    }
}

#[inline]
fn get_input() -> Option<String> {
    std::env::args().nth(1).or_else(get_pipe)
}

#[inline]
fn get_stdin() -> Option<String> {
    let mut res = String::new();
    std::io::stdin()
        .read_line(&mut res)
        .ok()
        .map(|_| res.trim().to_string())
}

#[inline]
fn get_pipe() -> Option<String> {
    if has_stdin_pipe() {
        get_stdin()
    } else {
        None
    }
}

#[inline]
fn has_stdin_pipe() -> bool {
    unsafe { libc::isatty(libc::STDIN_FILENO) == 0 }
}

fn add_to_clipboard(data: &str) -> Result<(), ()> {
    let cls = Class::get("NSPasteboard").ok_or(())?;
    let clipboard: *mut Object = unsafe { msg_send![cls, generalPasteboard] };
    let clipboard: Id<Object> = unsafe { Id::from_ptr(clipboard) };
    let string_array = NSArray::from_vec(vec![NSString::from_str(data)]);
    let _: usize = unsafe { msg_send![clipboard, clearContents] };
    let _: bool = unsafe { msg_send![clipboard, writeObjects: string_array] };
    Ok(())
}
