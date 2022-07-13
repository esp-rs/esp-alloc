//! A simple `no_std` heap allocator for RISC-V and Xtensa processors from
//! Espressif.
//!
//! Currently supports:
//! - ESP32
//! - ESP32-C3
//! - ESP32-S2
//! - ESP32-S3
//!
//! __NOTE:__ using this as your global allocator requires using Rust's
//! `nightly` release channel.

#![no_std]
#![cfg_attr(feature = "oom-handler", feature(alloc_error_handler))]

use core::{
    alloc::{GlobalAlloc, Layout},
    cell::RefCell,
    ptr::{self, NonNull},
};

use bare_metal::Mutex;
use linked_list_allocator::Heap;
#[cfg(target_arch = "riscv32")]
use riscv::interrupt;
#[cfg(target_arch = "xtensa")]
use xtensa_lx::interrupt;

#[cfg(feature = "oom-handler")]
#[alloc_error_handler]
fn oom(_: core::alloc::Layout) -> ! {
    panic!("Allocation failed, out of memory");
}

pub struct EspHeap {
    heap: Mutex<RefCell<Heap>>,
}

impl EspHeap {
    /// Crate a new UNINITIALIZED heap allocator
    ///
    /// You must initialize this heap using the
    /// [`init`](struct.EspHeap.html#method.init) method before using the
    /// allocator.
    pub const fn empty() -> EspHeap {
        EspHeap {
            heap: Mutex::new(RefCell::new(Heap::empty())),
        }
    }

    /// Initializes the heap
    ///
    /// This function must be called BEFORE you run any code that makes use of
    /// the allocator.
    ///
    /// `heap_bottom` is a pointer to the location of the bottom of the heap.
    ///
    /// `size` is the size of the heap in bytes.
    ///
    /// Note that:
    ///
    /// - The heap grows "upwards", towards larger addresses. Thus `end_addr`
    ///   must be larger than `start_addr`
    ///
    /// - The size of the heap is `(end_addr as usize) - (start_addr as usize)`.
    ///   The allocator won't use the byte at `end_addr`.
    ///
    /// # Safety
    ///
    /// Obey these or Bad Stuff will happen.
    ///
    /// - This function must be called exactly ONCE.
    /// - `size > 0`
    pub unsafe fn init(&self, heap_bottom: *mut u8, size: usize) {
        interrupt::free(|cs| self.heap.borrow(*cs).borrow_mut().init(heap_bottom, size));
    }

    /// Returns an estimate of the amount of bytes in use.
    pub fn used(&self) -> usize {
        interrupt::free(|cs| self.heap.borrow(*cs).borrow_mut().used())
    }

    /// Returns an estimate of the amount of bytes available.
    pub fn free(&self) -> usize {
        interrupt::free(|cs| self.heap.borrow(*cs).borrow_mut().free())
    }
}

unsafe impl GlobalAlloc for EspHeap {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        interrupt::free(|cs| {
            self.heap
                .borrow(*cs)
                .borrow_mut()
                .allocate_first_fit(layout)
                .ok()
                .map_or(ptr::null_mut(), |allocation| allocation.as_ptr())
        })
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        interrupt::free(|cs| {
            self.heap
                .borrow(*cs)
                .borrow_mut()
                .deallocate(NonNull::new_unchecked(ptr), layout)
        });
    }
}
