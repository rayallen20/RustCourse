fn main() {
    let x = 5;                  // ----------+-- 'b
    let r = &x;                // --+--      |  'a
    println!("r: {}", r);            // -----------+
}
