struct Inventory {
    harvests: u32,
    seeds: u32,
}

enum PlotState {
    Empty,
    Planted { days_growing: u32 },
}

struct Plot {
    state: PlotState,
}

impl Inventory {
    fn consume_seed(&mut self) -> bool {
        if self.seeds > 0 {
            self.seeds -= 1;
            return true;
        }
        false
    }
}

impl Plot {
    fn plant(&mut self) -> bool {
        match self.state {
            PlotState::Planted { .. } => false,
            PlotState::Empty => {
                self.state = PlotState::Planted { days_growing: 0 };
                true
            }
        }
    }

    fn advance_day(&mut self) -> bool {
        match &mut self.state {
            PlotState::Planted { days_growing } => {
                *days_growing += 1;
                true
            }
            PlotState::Empty => false,
        }
    }

    fn harvest(&mut self) -> bool {
        match self.state {
            PlotState::Empty => false,
            PlotState::Planted { days_growing } => {
                if days_growing < 3 {
                    return false;
                }
                self.state = PlotState::Empty;
                true
            }
        }
    }
}

fn plant_seed(inventory: &mut Inventory, plot: &mut Plot) -> bool {
    match plot.state {
        PlotState::Planted { .. } => false,

        PlotState::Empty => {
            let consume = inventory.consume_seed();

            if !consume {
                return false;
            }

            plot.plant()
        }
    }
}

fn harvest_crop(inventory: &mut Inventory, plot: &mut Plot) -> bool {
    if !plot.harvest() {
        return false;
    }

    inventory.harvests += 1;
    true
}

fn main() {
    let mut inventory_with_seeds = Inventory {
        seeds: 2,
        harvests: 0,
    };

    let mut inventory_without_seeds = Inventory {
        seeds: 0,
        harvests: 0,
    };

    let mut plot = Plot {
        state: PlotState::Empty,
    };

    println!(
        "{} should be false",
        plant_seed(&mut inventory_without_seeds, &mut plot)
    );
    println!(
        "{} should be true",
        plant_seed(&mut inventory_with_seeds, &mut plot)
    );
    println!(
        "{} should be false",
        plant_seed(&mut inventory_with_seeds, &mut plot)
    );
    plot.advance_day();
    plot.advance_day();

    harvest_crop(&mut inventory_with_seeds, &mut plot);
    println!("{}", inventory_with_seeds.harvests);
    plot.advance_day();
    harvest_crop(&mut inventory_with_seeds, &mut plot);
    println!("{}", inventory_with_seeds.harvests);

    match &plot.state {
        PlotState::Planted { days_growing } => {
            println!("days_growing: {days_growing}");
        }
        PlotState::Empty => {
            println!("Shouldn't be printed");
        }
    }
}
