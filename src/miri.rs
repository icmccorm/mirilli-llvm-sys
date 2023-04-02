/* automatically generated by rust-bindgen 0.64.0 */
use crate::execution_engine::LLVMGenericValueRef;
use crate::prelude::LLVMTypeRef;

#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct PointerMetadata {
    pub alloc_id: ::std::os::raw::c_ulonglong,
    pub tag: ::std::os::raw::c_ulonglong,
    pub offset: ::std::os::raw::c_ulonglong,
}
#[test]
fn bindgen_test_layout_PointerMetadata() {
    const UNINIT: ::std::mem::MaybeUninit<PointerMetadata> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<PointerMetadata>(),
        24usize,
        concat!("Size of: ", stringify!(PointerMetadata))
    );
    assert_eq!(
        ::std::mem::align_of::<PointerMetadata>(),
        8usize,
        concat!("Alignment of ", stringify!(PointerMetadata))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).alloc_id) as usize - ptr as usize },
        0usize,
        concat!("Offset of field: ", stringify!(PointerMetadata), "::", stringify!(alloc_id))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).tag) as usize - ptr as usize },
        8usize,
        concat!("Offset of field: ", stringify!(PointerMetadata), "::", stringify!(tag))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).offset) as usize - ptr as usize },
        16usize,
        concat!("Offset of field: ", stringify!(PointerMetadata), "::", stringify!(offset))
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TrackedPointer {
    pub Pointer: *mut ::std::os::raw::c_void,
    pub Metadata: PointerMetadata,
}
impl Default for TrackedPointer {
    fn default() -> Self {
        TrackedPointer {
            Pointer: std::ptr::null_mut() as *mut ::std::os::raw::c_void,
            Metadata: PointerMetadata::default(),
        }
    }
}

impl TrackedPointer {
    pub fn from_parts(p: *mut ::std::os::raw::c_void, m: PointerMetadata) -> Self {
        TrackedPointer { Pointer: p, Metadata: m }
    }
}

#[test]
fn bindgen_test_layout_TrackedPointer() {
    const UNINIT: ::std::mem::MaybeUninit<TrackedPointer> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<TrackedPointer>(),
        32usize,
        concat!("Size of: ", stringify!(TrackedPointer))
    );
    assert_eq!(
        ::std::mem::align_of::<TrackedPointer>(),
        8usize,
        concat!("Alignment of ", stringify!(TrackedPointer))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).Pointer) as usize - ptr as usize },
        0usize,
        concat!("Offset of field: ", stringify!(TrackedPointer), "::", stringify!(Pointer))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).Metadata) as usize - ptr as usize },
        8usize,
        concat!("Offset of field: ", stringify!(TrackedPointer), "::", stringify!(Metadata))
    );
}

pub type MiriInterpCxOpaque = ::std::os::raw::c_void;

pub type MiriAllocationHook = ::std::option::Option<
    unsafe extern "C" fn(arg1: *mut MiriInterpCxOpaque, arg2: ::libc::size_t) -> TrackedPointer,
>;
pub type MiriFreeHook = ::std::option::Option<
    unsafe extern "C" fn(arg1: *mut MiriInterpCxOpaque, arg2: TrackedPointer),
>;
pub type MiriLoadStoreHook = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *mut MiriInterpCxOpaque,
        arg2: LLVMGenericValueRef,
        arg3: TrackedPointer,
        arg4: LLVMTypeRef,
    ),
>;
pub type MiriCallbackHook = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *mut MiriInterpCxOpaque,
        arg2: LLVMGenericValueRef,
        arg3: ::libc::size_t,
        arg4: *const ::std::os::raw::c_char,
        arg5: ::libc::size_t,
    ),
>;
