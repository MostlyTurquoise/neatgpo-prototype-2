use std::collections::{HashMap, HashSet};

pub struct TaskFlow<F: Fn() + ?Sized> {
    ni: u32,
    adjl: HashMap<u32, HashSet<u32>>,
    tmap: HashMap<u32, Task<F>>,
}

impl<F> TaskFlow<F>
where
    F: Fn() + ?Sized,
{
    pub fn register(&mut self, inf: Box<F>, deps: Option<&[u32]>) -> Result<u32, &str> {
        self.tmap.insert(self.ni, Task { body: inf });
        self.adjl.insert(self.ni, HashSet::new());
        if let Some(dep_list) = deps {
            for &dep in dep_list {
                if self.tmap.contains_key(&dep) {
                    self.adjl.get_mut(&self.ni).unwrap().insert(dep);
                }
            }
        }
        self.ni += 1;
        Ok(self.ni - 1)
    }

    pub fn list(&mut self) {
        for (key, _value) in &self.tmap {
            println!("{key}");
        }
    }

    pub fn add_dependency(&mut self, t1: u32, t2: u32) -> Result<(), &str> {
        match self.adjl.get_mut(&t1) {
            None => Err("t1 {t1} not in taskflow."),
            Some(li) => {
                if !self.tmap.contains_key(&t2) {
                    Err("t2 {t2} not in taskflow")
                } else if li.insert(t2) {
                    Ok(())
                } else {
                    Err("t2 {t2} already dependency")
                }
            }
        }
    }

    pub fn execute_from(&self, t: u32) -> Result<(), ()> {
        if self.tmap.contains_key(&t) {
            let mut to_run = Vec::<u32>::new();
            to_run.push(t);
            let mut run = HashSet::<u32>::new();
            while to_run.len() > 0 {
                let mut runnable = true;
                for dep in self.adjl.get(to_run.last().unwrap()).unwrap() {
                    if !run.contains(dep) {
                        runnable = false;
                        to_run.push(*dep);
                    }
                }
                if runnable {
                    let ttr = to_run.pop().unwrap();
                    (self.tmap.get(&ttr).unwrap().body)();
                    run.insert(ttr);
                }
            }

            Ok(())
        } else {
            Err(())
        }
    }

    pub fn new() -> TaskFlow<F> {
        TaskFlow {
            ni: 0,
            adjl: HashMap::<u32, HashSet<u32>>::new(),
            tmap: HashMap::<u32, Task<F>>::new(),
        }
    }
}

pub struct Task<F: Fn() + ?Sized> {
    body: Box<F>,
}
