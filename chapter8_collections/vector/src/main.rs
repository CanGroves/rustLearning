fn main() {
    let mut v: Vec<i32> = Vec::new();
    v.push(5);
    v.push(6);
    v.push(8);
    v.push(9);

    for i in &v {
        println!("{}", i);
    }

    let v2 = vec![1, 2, 3];
    let third: &i32 = &v2[2];
    println!("The third element is {}", third);
    // v2.push(6); // error! immutable borrow in line 8, can bottow it as mutable

    match v2.get(2) {
        Some(third) => println!("The third ele is {}", third),
        None => println!("There is no third element."),
    }

    let mut v_mut = vec![100, 32, 57];
    for i in &mut v_mut {
        *i += 50;
    }
}
