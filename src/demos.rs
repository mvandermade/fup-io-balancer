#![allow(unused)]  //TODO

#[derive(Debug, Clone, Copy)]
struct MyNr { nr: i32 }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let nr = MyNr { nr: 123 };
        number_eater(nr);
        number_eater(nr);  // works because of Copy
    }

    fn number_eater(_: MyNr) {}
}
