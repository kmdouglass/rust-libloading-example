use std::boxed::Box;
use std::ffi::OsStr;

use libc::c_int;
use libloading::{Library, Symbol};

#[repr(C)]
struct Object {
    _private: [u8; 0],
}
type Init = extern "C" fn() -> *mut Object;
type GetInfo = extern "C" fn(*const Object) -> c_int;
type SetInfo = extern "C" fn(*mut Object, c_int);

struct Plugin<'a> {
    get_info: Symbol<'a, GetInfo>,
    library: Box<Library>,
    object: *mut Object,
    set_info: Symbol<'a, SetInfo>,
}

impl<'a> Plugin<'a> {
    fn new(library_name: &OsStr) -> Plugin<'a> {
        let library = Box::new(Library::new(library_name).unwrap());
        let init: Symbol<Init> = unsafe { library.get(b"init\0").unwrap() };

        let object: *mut Object = init();

        let get_info: Symbol<GetInfo> = unsafe { library.get(b"get_info\0").unwrap() };
        let set_info: Symbol<SetInfo> = unsafe { library.get(b"set_info\0").unwrap() };

        Plugin {
            get_info: get_info,
            library: library,
            object: object,
            set_info: set_info,
        }
    }
}

fn main() {
    let library_path: &OsStr = OsStr::new("ffi-test/libffi-test.so");
    let plugin = Plugin::new(library_path);

    println!("Original value: {}", (plugin.get_info)(plugin.object));
    (plugin.set_info)(plugin.object, 42);

    println!("New value: {}", (plugin.get_info)(plugin.object));
}
