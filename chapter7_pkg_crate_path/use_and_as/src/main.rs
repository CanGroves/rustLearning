use std::collections::HashMap;

use std::fmt;
use std::io;

fn main() {
    let mut map = HashMap::new();
    map.insert(1,2);

}

// fn function1() -> fmt::Result {
//     // --snip--
// }

// fn function2() -> io::Result<()> {

// }

use std::fmt::Result;
use std::io::Result as IoResult;

// fn function1() -> Result {
//     // --snip--
// }

// fn function2() -> IoResult<()> {

// }