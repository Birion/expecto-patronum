use expecto_patronum::magic::{get_name, Caster};

fn main() {
    let name = get_name();
    let caster = Caster::new(name);
    caster.cast()
}
