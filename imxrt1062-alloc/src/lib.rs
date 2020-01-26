//! A global allocator for the iMXRT1062
//! 
//! # ABI
//! 
//! The linker must expose two symbols:
//! 
//! - `__sheap`, the start of heap
//! - `__eheap`, the end of heap
//! 
//! See the `imxrt1062-rt` crate for an implementation
//! of a runtime that defines these two symbols.

#![no_std]

use linked_list_allocator::LockedHeap;

#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();

/// Initialize the heap allocator
/// 
/// # Safety
/// 
/// This function registers the heap allocator. It may only be called once!
pub unsafe fn init() {
    extern "C" {
        static __sheap: u32;
        static __eheap: u32;
    }

    let heap_start = &__sheap as *const u32 as usize;
    let heap_end = &__eheap as *const u32 as usize;
    ALLOCATOR.lock().init(heap_start, heap_end - heap_start);
}