#![no_std]

use engine::Client as GameEngine;
use soroban_sdk::{contractimpl, BytesN, Env};

use crate::engine::{Direction, Point};

pub struct Solution;

mod engine {
    soroban_sdk::contractimport!(file = "../game_engine.wasm");
}

mod test;

#[contractimpl]
impl Solution {

    // Use "make build-optimized" for the smallest wasm file. Also I stripped the contractspecv0 section with wasm-strip tool. My final file size is 739 bytes

    pub fn solve(env: Env, engine_id: BytesN<32>) {
        let engine = GameEngine::new(&env, &engine_id);

        let mut p_pos: Point = Point(8, 8);
        while p_pos.0 < 300 {
            let map = engine.get_map().keys();
            if map.is_empty() {
                p_pos.0 = p_pos.0.wrapping_add(6);
                Self::change_gal(&engine);
                continue;
            }
            Self::move_ship(&engine, &mut map.get_unchecked(0).unwrap(), &mut p_pos);
        }
    }

    fn change_gal(engine: &GameEngine) {
        engine.p_turn(&Direction::Right);
        engine.p_move(&Some(6));
    }

    fn turn(engine: &GameEngine, val1: i32, val2: i32, direction1: &Direction, direction2: &Direction) {
        let mut direction = direction2;
        if val1 < val2 {
            direction = direction1;
        }
        engine.p_turn(direction);
    }

    fn move_ship(engine: &GameEngine, target: &mut Point, p_pos: &mut Point) {
        let mut mask1 = (*p_pos).1.wrapping_sub(target.1) >> 31;
        let mut r1 = ((*p_pos).1).wrapping_sub(target.1).wrapping_add(mask1) ^ mask1;
        Self::turn(&engine, p_pos.1, target.1, &Direction::Up, &Direction::Down);
        engine.p_move(&Some(r1 as u32));
        Self::turn(&engine, p_pos.0, target.0, &Direction::Right, &Direction::Left);
        mask1 = (*p_pos).0.wrapping_sub(target.0) >> 31;
        r1 = ((*p_pos).0).wrapping_sub(target.0).wrapping_add(mask1) ^ mask1;
        engine.p_move(&Some(r1 as u32));
        (*p_pos).0 = target.0;
        (*p_pos).1 = target.1;
        engine.p_harvest();
        engine.p_shoot();
    }
}
