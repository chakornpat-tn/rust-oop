trait Weapon {
    fn attack(&self);
}

struct Sword;
impl Weapon for Sword {
    fn attack(&self) {
        println!("Sword attack!");
    }
}

struct Staff;
impl Weapon for Staff {
    fn attack(&self) {
        println!("Staff attack!");
    }
}

struct SwordMan {
    health: u8,
    strength: u8,
    intelligence: u8,
    weapon: Box<dyn Weapon>,
}

impl SwordMan {
    fn new() -> Self {
        Self {
            health: 100,
            strength: 20,
            intelligence: 10,
            weapon: Box::new(Sword),
        }
    }

    fn health_increase(&mut self, value: u8) {
        if self.health + value > 100 {
            self.health = 100;
            return;
        }
        self.health += value;
    }

    fn health_decrease(&mut self, value: u8) {
        self.health = self.health.saturating_sub(value);
    }
}

struct Mage {
    health: u8,
    strength: u8,
    intelligence: u8,
    weapon: Box<dyn Weapon>,
}
impl Mage {
    fn new() -> Self {
        Self {
            health: 80,
            strength: 10,
            intelligence: 20,
            weapon: Box::new(Staff),
        }
    }

    fn health_increase(&mut self, value: u8) {
        if self.health + value > 80 {
            self.health = 80;
            return;
        }
        self.health += value;
    }

    fn health_decrease(&mut self, value: u8) {
        self.health = self.health.saturating_sub(value);
    }
}

struct Healer {
    health: u8,
    strength: u8,
    intelligence: u8,
    weapon: Box<dyn Weapon>,
}

impl Healer {
    fn new() -> Self {
        Self {
            health: 70,
            strength: 5,
            intelligence: 14,
            weapon: Box::new(Staff),
        }
    }
    fn health_increase(&mut self, value: u8) {
        if self.health + value > 70 {
            self.health = 70;
            return;
        }
        self.health += value;
    }
    fn health_decrease(&mut self, value: u8) {
       self.health = self.health.saturating_sub(value);
    }
    
}

fn  use_attack(weapon:Box<dyn Weapon>) {
    weapon.attack();
}

fn main() {
    let mut player1 = SwordMan::new();
    let mut player2 = Mage::new();
    let mut player3 = Healer::new();

    player1.health_decrease(10);
    player2.health_decrease(10);
    player3.health_decrease(10);

    println!("Player 1 health: {}", player1.health);
    println!("Player 2 health: {}", player2.health);
    println!("Player 3 health: {}", player3.health);

    player1.health_increase(10);
    player2.health_increase(5);
    player3.health_increase(20);

    println!("Player 1 health: {}", player1.health);
    println!("Player 2 health: {}", player2.health);
    println!("Player 3 health: {}", player3.health);

    use_attack(player1.weapon);
    use_attack(player2.weapon);
    use_attack(player3.weapon);


}
