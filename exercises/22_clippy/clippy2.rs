fn main() {
    let mut res = 42;
    let option = Some(12);

    if let Some(_name_doesnt_matter_here) = option {
        res += 1;
    }

    println!("{res}");
}
