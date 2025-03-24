fn noop() {}

fn main() {
    let fn_ptr = noop as usize;

    println!("noop in usise: 0x{:x}", fn_ptr);
}
