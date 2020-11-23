pub mod magic {
    const ANIMALS: &[&str; 132] = &[
        "aardvark",
        "adder",
        "alligator",
        "anaconda",
        "anteater",
        "antelope",
        "arctic wolf",
        "armadillo",
        "badger",
        "bat",
        "beaver",
        "beetle",
        "bison",
        "black bear",
        "boa",
        "bongo",
        "bonobo",
        "brown bear",
        "buffalo",
        "camel",
        "cassowary",
        "cat",
        "chameleon",
        "cheetah",
        "chicken",
        "chimpanzee",
        "chipmunk",
        "clam",
        "cockatoo",
        "cow",
        "coyote",
        "crab",
        "crocodile",
        "deer",
        "desert fox",
        "dingo",
        "dog",
        "dolphin",
        "donkey",
        "duck",
        "eagle",
        "eel",
        "elephant",
        "ferret",
        "fish",
        "flamingo",
        "fox",
        "frog",
        "gazelle",
        "gharial",
        "giant tortoise",
        "giraffe",
        "goat",
        "gorilla",
        "grizzly bear",
        "groundhog",
        "guinea pig",
        "hedgehog",
        "hen",
        "hippo",
        "hippopotamus",
        "horse",
        "hyena",
        "iguana",
        "jaguar",
        "kangaroo",
        "koala",
        "komodo dragon",
        "lemur",
        "leopard",
        "lion",
        "lizard",
        "llama",
        "lobster",
        "lynx",
        "macaque",
        "mandrill",
        "mole",
        "monitor",
        "moose",
        "mouse",
        "nyala",
        "octopus",
        "okapi",
        "orangutan",
        "ostrich",
        "panda",
        "panda bear",
        "pangolin",
        "panther",
        "parrot",
        "peafowl",
        "pig",
        "poison frog",
        "polar bear",
        "rabbit",
        "raccoon",
        "rattlesnake",
        "red panda",
        "reindeer",
        "rhinoceros",
        "ring-tailed lemur",
        "salmon",
        "scorpion",
        "sea lion",
        "seagull",
        "seahorse",
        "shark",
        "sheep",
        "shrimp",
        "slug",
        "snail",
        "snow leopard",
        "spider",
        "squid",
        "squirrel",
        "starfish",
        "swan",
        "tapir",
        "tarantula",
        "tiger",
        "timber wolf",
        "turkey",
        "turtle",
        "warthog",
        "weasel",
        "whale",
        "wildcat",
        "wildebeest",
        "wolf",
        "wombat",
        "zebra",
    ];

    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    use std::{env, process};

    pub struct Caster {
        hash: u64,
    }

    impl Caster {
        pub fn new() -> Self {
            let name = get_name();
            let hash = hash_username(&name);
            Caster { hash }
        }

        fn get_animal(&self) -> &'static str {
            let len = ANIMALS.len();
            ANIMALS[self.hash as usize % len]
        }

        pub fn cast(&self) {
            let animal = self.get_animal();
            println!(
                "You raise your wand and intone, \"Expecto patronum!\" \
                A wisp of smoke escapes from the tip of your wand and turns into a silvery {}.",
                animal
            );
        }
    }

    fn get_name() -> String {
        match env::var("USERNAME") {
            Ok(user) => user,
            Err(_) => {
                println!("Username cannot be determined!");
                process::exit(1)
            }
        }
    }

    fn hash_username(user: &str) -> u64 {
        let mut hasher = DefaultHasher::new();
        user.hash(&mut hasher);
        hasher.finish()
    }
}
