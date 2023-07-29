#![no_std]

use engine::{Client as GameEngine};
use soroban_sdk::{contractimpl, BytesN, Env};
use engine::MapElement;

use crate::engine::Point;
use crate::engine::Direction;

pub struct Solution;

mod engine {
    soroban_sdk::contractimport!(file = "../game_engine.wasm");
}

mod test;

#[contractimpl]
impl Solution {
    pub fn solve(env: Env, engine_id: BytesN<32>) {
        let engine = GameEngine::new(&env, &engine_id);

        // YOUR CODE START
        loop{
            let mut currentGalaxyMap = engine.get_map();
            
            for galaxyObject in currentGalaxyMap.iter() {
                let galaxyObj = galaxyObject.unwrap();
                if engine.p_fuel() > 200 || (engine.p_fuel()  <= 200 && galaxyObj.1 == MapElement::FuelPod) {
                    Self::move_ship(&engine, galaxyObj.0, galaxyObj.1 == MapElement::FuelPod)
                }
            } 

            Self::change_gal(&engine);
            if engine.p_points() > 100 {
                break;
            }
        }
        
        // YOUR CODE END
    }

    fn change_gal(engine: &GameEngine){
        engine.p_turn(&engine::Direction::Right);
        engine.p_move(&Some(17));
    }

    fn turn(engine: &GameEngine, direction: &Direction){
        engine.p_turn(direction);
    }

    fn move_ship(engine: &GameEngine, target: Point, is_harvest: bool){
        if(engine.p_pos().0 - target.0 > 0 && engine.p_pos().1 - target.1 > 0){
            Self::turn(&engine, &Direction::DownLeft);
            if (engine.p_pos().0 - target.0).abs() <= (engine.p_pos().1 - target.1).abs() {
                engine.p_move(&Some((engine.p_pos().0 - target.0).abs() as u32));
            }
            else{
                engine.p_move(&Some((engine.p_pos().1 - target.1).abs() as u32));
            }
            if(engine.p_pos().0 - target.0 > 0){
                Self::turn(&engine, &Direction::Left);
                engine.p_move(&Some((engine.p_pos().0 - target.0).abs() as u32));
            }
            else if(engine.p_pos().1 - target.1 > 0){
                Self::turn(&engine, &Direction::Down);
                engine.p_move(&Some((engine.p_pos().1 - target.1).abs() as u32));
            }
        }
        else if(engine.p_pos().0 - target.0 > 0 && engine.p_pos().1 - target.1 == 0){
            Self::turn(&engine, &Direction::Left);
            engine.p_move(&Some((engine.p_pos().0 - target.0).abs() as u32));
        }
        else if(engine.p_pos().0 - target.0 > 0 && engine.p_pos().1 - target.1 < 0){
            Self::turn(&engine, &Direction::UpLeft);
            if (engine.p_pos().0 - target.0).abs() <= (engine.p_pos().1 - target.1).abs() {
                engine.p_move(&Some((engine.p_pos().0 - target.0).abs() as u32));
            }
            else{
                engine.p_move(&Some((engine.p_pos().1 - target.1).abs() as u32));
            }
            if(engine.p_pos().0 - target.0 > 0){
                Self::turn(&engine, &Direction::Left);
                engine.p_move(&Some((engine.p_pos().0 - target.0).abs() as u32));
            }
            else if(engine.p_pos().1 - target.1 < 0){
                Self::turn(&engine, &Direction::Up);
                engine.p_move(&Some((engine.p_pos().1 - target.1).abs() as u32));
            }
        }
        else if(engine.p_pos().0 - target.0 == 0 && engine.p_pos().1 - target.1 < 0){
            Self::turn(&engine, &Direction::Up);
            engine.p_move(&Some((engine.p_pos().1 - target.1).abs() as u32));
        }
        else if(engine.p_pos().0 - target.0 < 0 && engine.p_pos().1 - target.1 < 0){
            Self::turn(&engine, &Direction::UpRight);
            if (engine.p_pos().0 - target.0).abs() <= (engine.p_pos().1 - target.1).abs() {
                engine.p_move(&Some((engine.p_pos().0 - target.0).abs() as u32));
            }
            else{
                engine.p_move(&Some((engine.p_pos().1 - target.1).abs() as u32));
            }
            if(engine.p_pos().0 - target.0 < 0){
                Self::turn(&engine, &Direction::Right);
                engine.p_move(&Some((engine.p_pos().0 - target.0).abs() as u32));
            }
            else if(engine.p_pos().1 - target.1 < 0){
                Self::turn(&engine, &Direction::Up);
                engine.p_move(&Some((engine.p_pos().1 - target.1).abs() as u32));
            }
        }
        else if(engine.p_pos().0 - target.0 < 0 && engine.p_pos().1 - target.1 == 0){
            Self::turn(&engine, &Direction::Right);
            engine.p_move(&Some((engine.p_pos().0 - target.0).abs() as u32));
        }
        else if(engine.p_pos().0 - target.0 < 0 && engine.p_pos().1 - target.1 > 0){
            Self::turn(&engine, &Direction::DownRight);
            if (engine.p_pos().0 - target.0).abs() <= (engine.p_pos().1 - target.1).abs() {
                engine.p_move(&Some((engine.p_pos().0 - target.0).abs() as u32));
            }
            else{
                engine.p_move(&Some((engine.p_pos().1 - target.1).abs() as u32));
            }
            if(engine.p_pos().0 - target.0 < 0){
                Self::turn(&engine, &Direction::Right);
                engine.p_move(&Some((engine.p_pos().0 - target.0).abs() as u32));
            }
            else if(engine.p_pos().1 - target.1 > 0){
                Self::turn(&engine, &Direction::Down);
                engine.p_move(&Some((engine.p_pos().1 - target.1).abs() as u32));
            }
        }
        else if(engine.p_pos().0 - target.0 == 0 && engine.p_pos().1 - target.1 > 0){
            Self::turn(&engine, &Direction::Down);
            engine.p_move(&Some((engine.p_pos().1 - target.1).abs() as u32));
        }
        match is_harvest {
            true => engine.p_harvest(),
            false => engine.p_shoot()
        }
    }
}
