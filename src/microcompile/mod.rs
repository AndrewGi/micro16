/*
TODO: maybe mark unsafe?
Currently, transmute_slice requires Copy so it'll
only work with bitwise objects (hopefully)
*/
pub fn transmute_slice<T: Copy>(slice: &[u8]) -> &T {
    assert!(slice.len() >= std::mem::size_of::<T>());
    unsafe { std::mem::transmute(slice.as_ptr()) }
}
pub fn transmute_slice_mut<T: Copy>(slice: &mut [u8]) -> &mut T {
    assert!(slice.len() >= std::mem::size_of::<T>());
    unsafe { std::mem::transmute(slice.as_mut_ptr()) }
}
pub mod assembler;
pub mod asm_scanner;
pub mod instructions;