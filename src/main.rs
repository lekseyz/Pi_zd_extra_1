fn divByOne(a: u32) -> u32 {
    return a / 1;
}

fn main() {
    println!("grate answer: {}", divByOne(42)); 
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test() {
        assert_eq!(divByOne(42), 42);
        assert_eq!(divByOne(4), 4);
        assert_eq!(divByOne(2), 2);
    }
}
