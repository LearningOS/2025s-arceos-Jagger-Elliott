#![no_std]

use core::ptr::NonNull;

use allocator::{AllocError, BaseAllocator, ByteAllocator, PageAllocator};

/// Early memory allocator
/// Use it before formal bytes-allocator and pages-allocator can work!
/// This is a double-end memory range:
/// - Alloc bytes forward
/// - Alloc pages backward
///
/// [ bytes-used | avail-area | pages-used ]
/// |            | -->    <-- |            |
/// start       b_pos        p_pos       end
///
/// For bytes area, 'count' records number of allocations.
/// When it goes down to ZERO, free bytes-used area.
/// For pages area, it will never be freed!
///
pub struct EarlyAllocator {
    start: usize,
    end: usize,
    count: usize,
    byte_pos: usize,
    page_pos: usize,
    page_count: usize,
}

impl EarlyAllocator {
    fn align_up(&self, addr: usize, align: usize) -> usize {
        assert!(align.is_power_of_two(), "align must be a power of 2");
        (addr + align - 1) & !(align - 1)
    }

    fn align_down(&self, addr: usize, align: usize) -> usize {
        assert!(align.is_power_of_two());
        addr & !(align - 1)
    }
}

impl BaseAllocator for EarlyAllocator {
    fn init(&mut self, start: usize, size: usize) {
        self.start = start;
        self.end = start + size;
        self.count = 0;
        self.byte_pos = start;
        self.page_pos = self.end;
        self.page_count = 0;
    }

    fn add_memory(&mut self, _start: usize, size: usize) -> allocator::AllocResult {
        Ok(())
    }
}

impl ByteAllocator for EarlyAllocator {
    fn alloc(
        &mut self,
        layout: core::alloc::Layout,
    ) -> allocator::AllocResult<core::ptr::NonNull<u8>> {
        let start = self.align_up(self.byte_pos, layout.align());
        let end = start + layout.size();
        if end > self.page_pos {
            Err(AllocError::NoMemory)
        } else {
            self.byte_pos = end;
            self.count += 1;
            NonNull::new(start as *mut u8).ok_or(AllocError::NoMemory)
        }
    }

    fn dealloc(&mut self, _pos: core::ptr::NonNull<u8>, _layout: core::alloc::Layout) {
        self.count -= 1;
        if self.count == 0 {
            self.byte_pos = self.start;
        }
    }

    fn total_bytes(&self) -> usize {
        self.end - self.start
    }

    fn used_bytes(&self) -> usize {
        self.byte_pos - self.start
    }

    fn available_bytes(&self) -> usize {
        self.page_pos - self.byte_pos
    }
}

impl PageAllocator for EarlyAllocator {
    const PAGE_SIZE: usize = 4096;

    fn alloc_pages(
        &mut self,
        num_pages: usize,
        align_pow2: usize,
    ) -> allocator::AllocResult<usize> {
        assert_eq!(align_pow2 % PAGE_SIZE, 0);
        let start = self.align_down(self.page_pos, align_pow2);
        let end = start - num_pages * EarlyAllocator::PAGE_SIZE;

        if end < self.byte_pos {
            return Err(AllocError::NoMemory);
        } else {
            self.page_pos = end;
            return NonNull::new(end as *mut usize).ok_or(AllocError::NoMemory);
        }
    }

    fn dealloc_pages(&mut self, _pos: usize, _num_pages: usize) {}

    fn total_pages(&self) -> usize {
        (self.end - self.start) / EarlyAllocator::PAGE_SIZE
    }

    fn used_pages(&self) -> usize {
        (self.end - self.page_pos) / EarlyAllocator::PAGE_SIZE
    }

    fn available_pages(&self) -> usize {
        (self.page_pos - self.byte_pos) / EarlyAllocator::PAGE_SIZE
    }
}
