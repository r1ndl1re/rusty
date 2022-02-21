fn greet_world() {
    println!("Hello, world!");
    let japan = "ハローワールド";
    let japan2 = "ハローworld";
    let regions = [japan, japan2];
    for region in regions.iter() {
        println!("{}", &region);
    }
}

fn main() {
    greet_world();
}
