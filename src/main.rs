use expecto_patronum::{get_name, Caster};

fn main() {
    let caster: Caster = get_name().into();
    caster.cast()
}
