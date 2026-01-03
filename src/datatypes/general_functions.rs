pub fn align_memory(mem : usize, alignment : usize) -> usize {
    return (mem + (alignment - 1)) & !(alignment - 1);
}
