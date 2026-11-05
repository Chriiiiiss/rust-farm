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
    fn new() -> Self {
        Plot {
            state: PlotState::Empty,
        }
    }

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

struct Farm {
    inventory: Inventory,
    plots: Vec<Plot>,
}

impl Farm {
    fn new(inventory: Inventory, plots: Vec<Plot>) -> Self {
        Farm { inventory, plots }
    }

    fn advance_day(&mut self) {
        for plot in &mut self.plots {
            plot.advance_day();
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
    let inventory_with_seeds = Inventory {
        seeds: 5,
        harvests: 0,
    };
    let plots = vec![Plot::new(), Plot::new(), Plot::new()];

    let mut farm = Farm::new(inventory_with_seeds, plots);

    plant_seed(&mut farm.inventory, &mut farm.plots[0]);
    plant_seed(&mut farm.inventory, &mut farm.plots[1]);
    farm.advance_day();
    farm.advance_day();
    farm.advance_day();
    harvest_crop(&mut farm.inventory, &mut farm.plots[0]);

    for (i, plot) in farm.plots.iter().enumerate() {
        match plot.state {
            PlotState::Empty => println!("La parcelle {} est vide", i + 1),
            PlotState::Planted { days_growing } => {
                println!("La parcelle est planté et est à son {days_growing} jours")
            }
        }
    }
}
