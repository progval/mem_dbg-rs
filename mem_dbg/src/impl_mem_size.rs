/*
 * SPDX-FileCopyrightText: 2023 Inria
 *
 * SPDX-License-Identifier: Apache-2.0 OR LGPL-2.1-or-later
 */

use core::marker::PhantomData;

use crate::MemSize;

macro_rules! impl_memory_size {
    ($($ty:ty),*) => {$(
impl MemSize for $ty {
    #[inline(always)]
    fn mem_size(&self) -> usize {
        core::mem::size_of::<Self>()
    }
}
    )*};
}

impl_memory_size! {
   (), bool, char, f32, f64,
   u8, u16, u32, u64, u128, usize,
   i8, i16, i32, i64, i128, isize
}

impl<T> MemSize for &'_ T {
    #[inline(always)]
    fn mem_size(&self) -> usize {
        core::mem::size_of::<Self>()
    }
}

impl<T> MemSize for &'_ mut T {
    #[inline(always)]
    fn mem_size(&self) -> usize {
        core::mem::size_of::<Self>()
    }
}

impl<T: MemSize> MemSize for Option<T> {
    #[inline(always)]
    fn mem_size(&self) -> usize {
        core::mem::size_of::<Self>()
            + self
                .as_ref()
                .map_or(0, |x| x.mem_size() - core::mem::size_of::<T>())
    }

    #[inline(always)]
    fn mem_cap(&self) -> usize {
        core::mem::size_of::<Self>()
            + self
                .as_ref()
                .map_or(0, |x| x.mem_cap() - core::mem::size_of::<T>())
    }
}

impl<T: MemSize, const N: usize> MemSize for [T; N] {
    #[inline(always)]
    fn mem_size(&self) -> usize {
        core::mem::size_of::<Self>()
            + self
                .iter()
                .map(|x| x.mem_size() - core::mem::size_of::<T>())
                .sum::<usize>()
    }
}

#[cfg(all(feature = "alloc", not(feature = "std")))]
use alloc::vec::Vec;
#[cfg(feature = "alloc")]
impl<T: MemSize> MemSize for Vec<T> {
    #[inline(always)]
    fn mem_size(&self) -> usize {
        core::mem::size_of::<Self>() + self.iter().map(|x| x.mem_size()).sum::<usize>()
    }
    #[inline(always)]
    fn mem_cap(&self) -> usize {
        core::mem::size_of::<Self>()
            + self.iter().map(|x| x.mem_cap()).sum::<usize>()
            + (self.capacity() - self.len()) * core::mem::size_of::<T>()
    }
}

#[cfg(all(feature = "alloc", not(feature = "std")))]
use alloc::boxed::Box;
#[cfg(feature = "alloc")]
impl<T: MemSize> MemSize for Box<[T]> {
    #[inline(always)]
    fn mem_size(&self) -> usize {
        core::mem::size_of::<Self>() + self.iter().map(|x| x.mem_size()).sum::<usize>()
    }
}

#[cfg(all(feature = "alloc", not(feature = "std")))]
use alloc::boxed::Box;
#[cfg(feature = "alloc")]
impl<T: MemSize> MemSize for [T] {
    #[inline(always)]
    fn mem_size(&self) -> usize {
        core::mem::size_of::<usize>() + self.iter().map(|x| x.mem_size()).sum::<usize>()
    }
}

impl<T: ?Sized> MemSize for PhantomData<T> {
    #[inline(always)]
    fn mem_size(&self) -> usize {
        0
    }
}

impl MemSize for str {
    #[inline(always)]
    fn mem_size(&self) -> usize {
        core::mem::size_of::<usize>() + self.len()
    }
}

impl MemSize for String {
    #[inline(always)]
    fn mem_size(&self) -> usize {
        core::mem::size_of::<Self>() + self.len()
    }

    fn mem_cap(&self) -> usize {
        core::mem::size_of::<Self>() + self.capacity()
    }
}

#[cfg(feature = "mmap_rs")]
impl MemSize for mmap_rs::Mmap {
    #[inline(always)]
    fn mem_size(&self) -> usize {
        core::mem::size_of::<Self>()
    }
}

#[cfg(feature = "mmap_rs")]
impl MemSize for mmap_rs::MmapMut {
    #[inline(always)]
    fn mem_size(&self) -> usize {
        core::mem::size_of::<Self>()
    }
}
