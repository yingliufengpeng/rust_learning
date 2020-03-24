fn main() {
    // use getaver;
    let mut m = getaver::AverCollect::new();

    m.add(3);
    m.add(4);
    m.add(4);

    println!("avg is {}", m.get_aver());
    m.remove();
    println!("avg is {}", m.get_aver());



}