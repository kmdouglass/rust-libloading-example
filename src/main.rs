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
    library: Library,
    object: *mut Object,
    set_info: Symbol<'a, SetInfo>,
}

fn main() {
    let library = Library::new("ffi-test/libffi-test.so").unwrap();
    let init: Symbol<Init> = unsafe { library.get(b"init\0").unwrap() };

    let object: *mut Object = init();

    let get_info: Symbol<GetInfo> = unsafe { library.get(b"get_info\0").unwrap() };
    let set_info: Symbol<SetInfo> = unsafe { library.get(b"set_info\0").unwrap() };

    let plugin = Plugin {
        get_info: get_info,
        library: library,
        object: object,
        set_info: set_info,
    };

    println!("Original value: {}", (*plugin.get_info)(plugin.object));
    (*plugin.set_info)(plugin.object, 42);
    
    println!("New value: {}", (*plugin.get_info)(plugin.object));
}
