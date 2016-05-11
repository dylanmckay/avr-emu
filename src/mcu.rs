use Core;
use addons;

pub struct Mcu
{
    core: Core,
    addons: Vec<Box<addons::Addon>>,
}

impl Mcu
{
    pub fn new(core: Core) -> Self {
        Mcu {
            core: core,
            addons: Vec::new(),
        }
    }

    pub fn attach(&mut self, addon: Box<addons::Addon>) {
        self.addons.push(addon);
    }

    pub fn tick(&mut self) {
        self.core.tick();

        for addon in self.addons.iter_mut() {
            addon.tick(&mut self.core);
        }
    }

    pub fn core(&self) -> &Core { &self.core }
}