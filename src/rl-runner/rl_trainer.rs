extern crate rand;

use rand::distributions::uniform::UniformInt;
use rand::thread_rng;

pub trait PolicySpace {
    type Reward;

    fn initial_state() -> Self;
    fn choices(&self) -> Vec<Self>;
    fn is_end(&self) -> bool;
    fn get_reward(states: &Vec<Self>) -> Reward;
}

type PolicyRecord<T: PolicySpace> = (Vec<T>, <T as PolicyState>::Reward);

pub fn randomly_sample_policies<T: PolicySpace>(space: T, samples: u64) -> Vec<PolicyRecord> {
    let mut record: Vec<PolicyRecord> = Vec::new();
    for n in 0..samples {
        let mut current_path = Vec::new();
        current_path.push(T::initial_state());
        while !current_path[current_path.len() - 1].is_end() {
            let p = current_path[current_path.len() - 1].choices();
            let s = UniformInt::new(0, p.len() - 1);
            current_path.push(p[s.sample(&mut thread_rng())]);
        }
        let reward = T::get_reward(current_path);
        record.push((current_path, reward));
    }
    return record;
}

