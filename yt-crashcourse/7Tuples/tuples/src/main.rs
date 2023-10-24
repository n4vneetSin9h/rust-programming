fn main() {
    // Tuples
    let s_tuple = ("rice", "crab", "fish", "salad");

    let _i_tuple = (1, 2, 3, 4, 5);

    let m_tuple = ("Smith", "Laura", 4, 56, true, 5.45f32, 'c', (20, 40, 60));

    println!("{} {} is a {} year old lady, who is {}ft tall and loves to eat {} {} since the age of {}.", m_tuple.1, m_tuple.0, m_tuple.3, m_tuple.5, s_tuple.2, s_tuple.3, (m_tuple.7).0);

}
