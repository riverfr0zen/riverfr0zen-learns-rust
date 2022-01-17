#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}


pub mod intro {
    pub fn hellotext_from_lib() -> &'static str {
        "Hello from the lib"
    }
}

pub mod explore_bevy_examples;