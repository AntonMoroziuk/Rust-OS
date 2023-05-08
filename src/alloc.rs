use core::mem::size_of;

const HEAP_START: *const u32 = &STACK_TOP as *const u32;

fn align(x: usize) -> usize {
    (((x - 1) >> 2) << 2) + 4
}

const HEADER_SIZE: usize = size_of::<BlockHeader>();

const RAM_END: usize = 0x20008000;

const HEAP_SIZE: usize = RAM_END - HEAP_START as usize;

struct BlockHeader {
    size: usize,
    next: Option<&'static mut BlockHeader>,
    prev: Option<&'static mut BlockHeader>,
    free: bool,
    data: [u8],
}

unsafe fn split_block(block: &mut BlockHeader, size: usize) -> *mut u8 {
    if block.size <= size + 2 * HEADER_SIZE + align(1) {
        block.free = false;
        block.data.as_mut_ptr()
    } else {
        let temp = (block.data.as_mut_ptr() as *mut u8).add(size) as *mut BlockHeader;
        temp.write(BlockHeader {
            size: block.size - size - HEADER_SIZE,
            next: block.next.take(),
            prev: Some(block),
            free: true,
            data: [0; 0],
        });
        block.size = size;
        block.next = Some(&mut *temp);
        block.free = false;
        block.data.as_mut_ptr()
    }
}

static mut G_ALLOC: Option<&'static mut BlockHeader> = None;

unsafe fn malloc_init() {
    let block = BlockHeader {
        size: HEAP_SIZE - HEADER_SIZE,
        next: None,
        prev: None,
        free: true,
        data: [0; 0],
    };
    G_ALLOC = Some(&mut *(HEAP_START as *mut BlockHeader));
    *G_ALLOC.as_mut().unwrap() = block;
}

unsafe fn defragment_forward(ptr: &mut BlockHeader) {
    let mut cur = ptr.next.as_mut().unwrap();
    while let Some(cur_next) = cur.next {
        if cur.free {
            ptr.size += HEADER_SIZE + cur.size;
            ptr.next = Some(cur_next);
        } else {
            return;
        }
        cur = cur_next;
    }
}

unsafe fn defragment_backward(ptr: &mut BlockHeader) -> Option<&'static mut BlockHeader> {
    let mut cur = ptr.prev.as_mut().unwrap();
    let mut tmp = ptr;
    while let Some(cur_prev) = cur.prev {
        if cur.free {
            cur.size += HEADER_SIZE + tmp.size;
            cur.next = tmp.next.take();
            tmp = cur;
            cur = cur_prev;
        } else {
            return Some(cur.next.as_mut().unwrap());
        }
    }
    Some(tmp)
}

pub unsafe fn alloc(size: usize) -> Option<*mut u8> {
    let size = align(size);
    if G_ALLOC.is_none() {
        alloc_init();
    }

    for mut cur in G_ALLOC.as_mut() {
        if cur.size >= size && cur.free {
            return Some(split_block(&mut *cur, size));
        }
    }
    None
}

pub unsafe fn dealloc(ptr: *mut u8) {
    let temp = (ptr as *mut BlockHeader).sub(1);
    if temp.free {
        return;
    } else {
        defragment(&mut *temp);
    }
}

unsafe fn defragment(ptr: &mut BlockHeader) {
    ptr.free = true;
    if let Some(block) = defragment_backward(ptr) {
        defragment_forward(&mut *block);
    }
}
