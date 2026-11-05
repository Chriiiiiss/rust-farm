struct Inventory {
    harvests: u32,
    seeds: u32,
}

#[derive(Debug)]
enum ActionError {
    InvalidPlot,
    NoSeeds,
    AlreadyPlanted,
    EmptyPlot,
    NotReady,
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

    fn plant(&mut self) -> Result<(), ActionError> {
        match self.state {
            PlotState::Planted { .. } => Err(ActionError::AlreadyPlanted),
            PlotState::Empty => {
                self.state = PlotState::Planted { days_growing: 0 };
                Ok(())
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

    fn harvest(&mut self) -> Result<(), ActionError> {
        match self.state {
            PlotState::Empty => Err(ActionError::EmptyPlot),
            PlotState::Planted { days_growing } => {
                if days_growing < 3 {
                    return Err(ActionError::NotReady);
                }
                self.state = PlotState::Empty;
                Ok(())
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

    fn plant(&mut self, index: usize) -> Result<(), ActionError> {
        match self.plots.get_mut(index) {
            Some(plot) => plant_seed(&mut self.inventory, plot),
            None => Err(ActionError::InvalidPlot),
        }
    }

    fn harvest(&mut self, index: usize) -> Result<(), ActionError> {
        match self.plots.get_mut(index) {
            Some(plot) => harvest_crop(&mut self.inventory, plot),
            None => Err(ActionError::InvalidPlot),
        }
    }

    fn print_plots(&self) {
        println!("===================================");
        for (i, plot) in self.plots.iter().enumerate() {
            match plot.state {
                PlotState::Empty => println!("La parcelle {} est vide", i + 1),
                PlotState::Planted { days_growing } => {
                    println!("La parcelle est planté et est à son {days_growing} jours")
                }
            }
        }
        println!("===================================");
    }
}

fn plant_seed(inventory: &mut Inventory, plot: &mut Plot) -> Result<(), ActionError> {
    match plot.state {
        PlotState::Planted { .. } => Err(ActionError::AlreadyPlanted),

        PlotState::Empty => {
            let consume = inventory.consume_seed();

            if !consume {
                return Err(ActionError::NoSeeds);
            }

            plot.plant()
        }
    }
}

fn harvest_crop(inventory: &mut Inventory, plot: &mut Plot) -> Result<(), ActionError> {
    plot.harvest()?;

    inventory.harvests += 1;
    Ok(())
}

fn main() {
    let inventory_with_seeds = Inventory {
        seeds: 5,
        harvests: 0,
    };
    let plots = vec![Plot::new(), Plot::new(), Plot::new()];

    let mut farm = Farm::new(inventory_with_seeds, plots);

    farm.print_plots();
    println!("{:?}", farm.plant(0));
    println!("{:?}", farm.plant(1));
    println!("{:?}", farm.plant(1));
    farm.print_plots();
    farm.advance_day();
    farm.advance_day();
    farm.print_plots();
    farm.advance_day();
    farm.print_plots();
    println!("{:?}", farm.harvest(1));
    println!("{:?}", farm.harvest(3));
    println!("{:?}", farm.plant(1));
    println!("{:?}", farm.plant(2));
}
