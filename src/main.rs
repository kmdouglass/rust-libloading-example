use std::ffi::OsStr;

use libc::c_int;
use libloading::os::unix::Symbol as RawSymbol;
use libloading::{Library, Symbol};

#[repr(C)]
struct Object {
    _private: [u8; 0],
}
type FreeObject = extern "C" fn(*mut Object);
type Init = extern "C" fn() -> *mut Object;
type GetInfo = extern "C" fn(*const Object) -> c_int;
type SetInfo = extern "C" fn(*mut Object, c_int);

struct Plugin {
    free_object: RawSymbol<FreeObject>,
    get_info: RawSymbol<GetInfo>,
    library: Library,
    object: *mut Object,
    set_info: RawSymbol<SetInfo>,
}

impl Plugin {
    unsafe fn new(library_name: &OsStr) -> Plugin {
        let library = Library::new(library_name).unwrap();
        let init: Symbol<Init> = library.get(b"init\0").unwrap();

        let object: *mut Object = init();

        let free_object: Symbol<FreeObject> = library.get(b"free_object\0").unwrap();
        let free_object = free_object.into_raw();
        let get_info: Symbol<GetInfo> = library.get(b"get_info\0").unwrap();
        let get_info = get_info.into_raw();
        let set_info: Symbol<SetInfo> = library.get(b"set_info\0").unwrap();
        let set_info = set_info.into_raw();

        Plugin {
            free_object: free_object,
            get_info: get_info,
            library: library,
            object: object,
            set_info: set_info,
        }
    }
}

impl Drop for Plugin {
    fn drop(&mut self) {
        (self.free_object)(self.object);
    }
}

fn main() {
    let library_path: &OsStr = OsStr::new("ffi-test/libffi-test.so");
    let plugin = unsafe { Plugin::new(library_path) };

    println!("Original value: {}", (plugin.get_info)(plugin.object));
    (plugin.set_info)(plugin.object, 42);

    println!("New value: {}", (plugin.get_info)(plugin.object));
}
