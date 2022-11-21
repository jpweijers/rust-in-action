fn main() {
    println!("Hello, world!");
    let dutch = "Hallo, wereld!";
    let french = "Bonjour!";
    let languages = [dutch, french];
    for language in languages.iter() {
        println!("{}", language);
    }
}
