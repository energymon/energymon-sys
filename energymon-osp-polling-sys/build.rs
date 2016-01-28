extern crate energymon_builder;

fn main() {
    energymon_builder::find_or_build("energymon-osp-polling").unwrap();
}
