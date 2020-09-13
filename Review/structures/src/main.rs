use std::collections::HashMap;

trait HaveParameter {}

trait Battle {
    fn action(&mut self, name: String);
    fn get_damege(&mut self, damege: f32);
    fn check_hp(&self) -> bool;
}

trait Movable {
    fn move_position(&mut self, x: f32, y: f32, z: f32);
}

#[derive(Debug, Clone)]
struct Parameter {
    name: String,
    hp: f32,
    mp: i32,
    actions: HashMap<String, i32>,
    position: (f32, f32, f32)
}

#[derive(Debug)]
struct Player {
    parameter: Parameter
}

impl HaveParameter for Player {}

impl Battle for Player {
    fn action(&mut self, name:String) {
        if let Some(point) = self.parameter.actions.get(&name) {
            self.parameter.mp -= point;
        }
    }
    fn get_damege(&mut self, damege: f32) {
        self.parameter.hp -= damege;
    }
    fn check_hp(&self) -> bool {
        self.parameter.hp >= 0.0
    }
}

impl Movable for Player {
    fn move_position(&mut self, x: f32, y: f32, z: f32) {
        self.parameter.position.0 += x;
        self.parameter.position.1 += y;
        self.parameter.position.2 += z;
    }
}

#[derive(Debug)]
struct Enemy {
    parameter: Parameter
}

impl HaveParameter for Enemy {}

impl Battle for Enemy {
    fn action(&mut self, name:String) {
        if let Some(point) = self.parameter.actions.get(&name) {
            self.parameter.mp -= point;
        }
    }
    fn get_damege(&mut self, damege: f32) {
        self.parameter.hp -= damege;
    }
    fn check_hp(&self) -> bool {
        self.parameter.hp >= 0.0
    }
}

fn main() {
    let player_parameter = Parameter {
        name: "Yusya".to_string(),
        hp: 100.0,
        mp: 12,
        actions: HashMap::<String, i32>::new(),
        position: (0.0, 0.0, 0.0)
    };
    let player = Player { parameter: player_parameter.clone() };
    println!("{:?}", player_parameter);
    println!("{:?}", player);
    println!("player is alive : {}", player.check_hp());
}
