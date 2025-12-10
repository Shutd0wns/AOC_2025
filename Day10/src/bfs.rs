use std::collections::{HashMap, HashSet, VecDeque};

pub struct BFS {
    start_state: String,
    end_state: String,
    buttons: Vec<Vec<u64>>
}
impl BFS {
    pub fn new(start: String, end: String, buttons: Vec<Vec<u64>>) -> BFS {
        BFS {
            start_state: start,
            end_state: end,
            buttons
        }
    }

    pub fn find_shortest(&self) -> Result<u64, String> {
        let mut queue: VecDeque<String> = VecDeque::new();
        let mut visited: HashSet<String> = HashSet::new();
        let mut parent: HashMap<String, String> = HashMap::new();

        parent.insert(self.start_state.clone(), "".to_string());
        queue.push_back(self.start_state.clone());
        visited.insert(self.start_state.clone());
        while !queue.is_empty() {
            let state = match queue.pop_front() {
                Some(val) => val,
                None => panic!("Unreachable code: CATASTROPHIC FAILURE")
            };

            if state == self.end_state {
                break;
            }

            for button in &self.buttons {
                let mut new_state = state.clone().into_bytes();
                for &indicator in button {
                    new_state[indicator as usize] = match new_state[indicator as usize] {
                        b'#' => b'.',
                        b'.' => b'#',
                        _ => return Err("Invalid character".to_string())
                    }
                }
                let new_state_string = match String::from_utf8(new_state) {
                    Ok(val) => val,
                    Err(e) => return Err(e.to_string())
                };

                if !visited.contains(&new_state_string) {
                    parent.insert(new_state_string.clone(), state.clone());
                    queue.push_back(new_state_string.clone())
                }
                visited.insert(new_state_string.clone());
            }
        }

        let mut length = 0;
        let mut current = parent[&self.end_state].clone();
        while current != "" {
            length += 1;
            current = parent[&current.clone()].clone();
        }

        Ok(length)
    }
}