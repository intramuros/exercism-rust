pub fn find() -> Option<u32> {
    (1..1000)
        .flat_map(|a| (1..(1000 - a)).map(move |b| (a,b,1000-a-b)))
        .filter(|(a, b, c)| c*c == a*a + b*b)
        .map(|(a, b, c)| a*b*c)
        .next()
}

