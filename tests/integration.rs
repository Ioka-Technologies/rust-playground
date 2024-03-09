use std::collections::HashMap;

use playground::lessons::borrow;
use cucumber::{gherkin::Step, given, then, when, World};

// `World` is your shared, likely mutable state.
// Cucumber constructs it via `Default::default()` for each scenario.
#[derive(Debug, Default, World)]
pub struct VecWorld {
    vec: Option<Vec<i32>>,
    initial_element_count: i32,
    alter_count: i32,
}

#[derive(Debug, Default, World)]
pub struct VecsWorld {
    vecs: HashMap<String, VecWorld>,
}

#[given(expr = "an initial vector with {int} elements")]
async fn setup_vec(world: &mut VecWorld, initial_element_count: i32) {
    world.alter_count = 0;
    world.initial_element_count = initial_element_count;
    world.vec = Some(vec![0; initial_element_count as usize]);
}

#[given(expr = "an initial vector with the following number of elements")]
async fn setup_vecs(world: &mut VecsWorld, step: &Step) {
    if let Some(table) = step.table.as_ref() {
        for row in table.rows.iter().skip(1) { // NOTE: skip header
            let num_elements_as_string = &row[0];

            let vec_world = world
                .vecs
                .entry(num_elements_as_string.clone())
                .or_insert(VecWorld::default());

            setup_vec(vec_world, num_elements_as_string.parse().unwrap()).await;
        }
    }
}

#[when(expr = "I attempt to alter the vector and specify {int} minimum elements")]
async fn alter_vec(world: &mut VecWorld, minimum: i32) {
    crate::borrow::alter_vec(&mut world.alter_count, minimum as usize, &mut world.vec);
}

#[when(expr = "I attempt to alter the vector with a given minimum")]
async fn alter_vecs(world: &mut VecsWorld, step: &Step) {
    if let Some(table) = step.table.as_ref() {
        for row in table.rows.iter().skip(1) { // NOTE: skip header
            let num_elements_as_string = &row[0];
            let minimum_as_string: &String = &row[1];

            let vec_world = world
                .vecs
                .entry(num_elements_as_string.clone())
                .or_insert(VecWorld::default());

            alter_vec(vec_world, minimum_as_string.parse().unwrap()).await;
        }
    }
}

#[then(expr = "the vector has {int} elements and has been altered the expected amount of times")]
async fn check_vec(world: &mut VecWorld, num_elements: i32) -> Result<(), &'static str> {
    (world.vec.as_ref().unwrap().len() == num_elements as usize).then_some(()).ok_or("Vector length does not match expected value")?;
    (world.alter_count == num_elements - world.initial_element_count).then_some(()).ok_or("Alter count does not match expected value")?;

    Ok(())
}

#[then(expr = "the vector has the following number of elements")]
async fn check_vecs(world: &mut VecsWorld, step: &Step) -> Result<(), &'static str> {
    if let Some(table) = step.table.as_ref() {
        for row in table.rows.iter().skip(1) { // NOTE: skip header
            let num_elements_as_string = &row[0];
            let expected_as_string: &String = &row[1];

            let vec_world = world
                .vecs
                .entry(num_elements_as_string.clone())
                .or_insert(VecWorld::default());

            check_vec(vec_world, expected_as_string.parse().unwrap()).await?;
        }
    }

    Ok(())
}

// This runs before everything else, so you can setup things here.
#[tokio::main]
async fn main() {
    VecWorld::run(
        "tests/features/simple.feature",
    ).await;
    VecsWorld::run(
        "tests/features/table.feature",
    ).await;
}
