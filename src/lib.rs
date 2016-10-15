//! Volatile access to memory mapped hardware registers
//!
//! # Usage
//!
//! ```
//! use volatile_register::{RO, RW, WO};
//!
//! /// A struct that represents the memory mapped register block for the GPIO
//! /// (General Purpose I/O) peripherals.
//! #[repr(C)]
//! pub struct Gpio {
//!     /// Control Register
//!     cr: RW<u32>,
//!     /// Input Data Register
//!     idr: RO<u32>,
//!     /// Output Data Register
//!     odr: WO<u32>,
//!     // .. more registers ..
//! }
//!
//! /// Accessor to the register block associated to the GPIOA peripheral
//! fn gpioa() -> &'static Gpio {
//!     const ADDRESS: usize = 0x40010800;
//!
//!     unsafe { &*(ADDRESS as *const Gpio) }
//! }
//!
//! /// Accessor to the register block associated to the GPIOC peripheral
//! /// NOTE(unsafe) This function hands out mutable aliases to a single address.
//! unsafe fn gpioc_mut() -> &'static mut Gpio {
//!     const ADDRESS: usize = 0x40011000;
//!
//!     unsafe { &mut *(ADDRESS as *mut Gpio) }
//! }
//! ```

#![deny(missing_docs)]
#![no_std]

use core::cell::UnsafeCell;
use core::ptr;

/// Read-Only register
#[repr(C)]
pub struct RO<T> {
    register: T,
}

impl<T> RO<T>
    where T: Copy
{
    /// Uninterruptible if `T` is a word, halfword or byte
    #[inline(always)]
    pub fn read(&self) -> T {
        unsafe { ptr::read_volatile(&self.register) }
    }
}

/// Read-Write register
#[repr(C)]
pub struct RW<T> {
    register: T,
}

impl<T> RW<T>
    where T: Copy
{
    /// Uninterruptible if `T` is a word, halfword or byte
    #[inline(always)]
    pub fn read(&self) -> T {
        unsafe { ptr::read_volatile(&self.register) }
    }

    /// Uninterruptible if `T` is a word, halfword or byte
    #[inline(always)]
    pub fn write(&mut self, value: T) {
        unsafe {
            ptr::write_volatile(&mut self.register, value);
        }
    }

    /// Perform a read-modify-write, using `func` to perform the modification.
    pub fn modify<F>(&mut self, func: F) where F: FnOnce(T) -> T {
        let mut t = self.read();
        t = func(t);
        self.write(t);
    }
}

/// Write-Only register
#[repr(C)]
pub struct WO<T> {
    register: UnsafeCell<T>,
}

impl<T> WO<T>
    where T: Copy
{
    /// Uninterruptible if `T` is a word, halfword or byte
    #[inline(always)]
    pub fn write(&self, value: T) {
        unsafe { ptr::write_volatile(self.register.get(), value) }
    }
}

unsafe impl<T> Sync for WO<T> where T: Sync {}
