struct MarineMonster{
    name:            String,
    depth:           u32,
    scientific_name: String
}

struct NormalFish{
    name:  String,
    depth: u32,
}

struct MarineFight<T : BreathUnderwater, R : BreathUnderwater>{
    creature_a: T,
    creature_b: R
}

enum SeaCreature{
    FISH(NormalFish),
    MONSTER(MarineMonster)
}

trait BreathUnderwater{
    fn breath(&self);
}

trait ExtractCreature{
    fn extract<T>( &self ) -> &T {
        match self {
            SeaCreature::FISH( creature ) | SeaCreature::MONSTER( creature ) => creature
        }
    }
}

impl ExtractCreature for SeaCreature{
}

impl BreathUnderwater for SeaCreature{
    fn breath(&self){
        match self{
            SeaCreature::FISH(fish)       => println!("{} says blub", fish.name),
            SeaCreature::MONSTER(monster) => println!("{} says hfffeeeee!", monster.name)
        }
    }
}

trait MarineBehaviours{
    fn attack(&self, creature: &SeaCreature);
    fn run(&self, creature: &SeaCreature);
}

trait Description{
    fn print_description(&self);
}

impl MarineMonster {
    fn new(name: String, depth: u32, scientific_name: String) -> Self {
        MarineMonster{name: name, depth: depth, scientific_name: scientific_name}
    }
}

impl MarineBehaviours for MarineMonster{
    fn attack(&self, creature: &SeaCreature){
        match creature{
            SeaCreature::FISH(fish)       => println!("{} attaks and kills {}", fish.name, self.name),
            SeaCreature::MONSTER(monster) => println!("{} attacks and came killed from {}", self.name, monster.name) 
        }
    }

    fn run(&self, creature: &SeaCreature){
        println!("{} is a marine monster! It never runs!", self.name);
    }
}

impl Description for MarineMonster{
    fn print_description(&self){
        println!( "The {} with the scientific name {} is a horrible monster that lives at water depth of {}m", &self.name, &self.scientific_name, &self.depth );
    }
}

impl NormalFish{
    fn new(name: String, depth: u32) -> Self {
        NormalFish{name: name, depth: depth}
    }
}

impl MarineBehaviours for NormalFish {
    fn run(&self, creature: &SeaCreature){
        match creature{
            SeaCreature::FISH(fish)       => println!("{} see {} and do nothing", self.name, fish.name),
            SeaCreature::MONSTER(monster) => println!("{} see {} and run away", self.name, monster.name) 
        }
    }

    fn attack(&self, creature: &SeaCreature){
        println!("{} should attacks? Are you fucking kidding me?", self.name);
    }
}

impl Description for NormalFish {
    fn print_description(&self){
        println!( "The {} is a simple fish that lives at water depth of {}m", &self.name, &self.depth );
    }
}

fn fight<T, R>(creature_a: &T, creature_b: &R) where T : BreathUnderwater,
                                                     R : BreathUnderwater{
    
}

fn run(creature_a: &SeaCreature, creature_b: &SeaCreature){
    creature_a.run(creature_b);
}

fn main() {
    let bobbit   = SeaCreature::MONSTER(MarineMonster::new("Bobbit", 40, "Eunice aphroditois"));
    let octopus  = SeaCreature::MONSTER(MarineMonster::new("Gigant Octopus", 100, "Enteroctopus"));
    let red_fish = SeaCreature::FISH(NormalFish::new("Normal Fish", 1));
    let carp     = SeaCreature::FISH(NormalFish::new("Carp", 2));

    fight( &carp, &red_fish );
}