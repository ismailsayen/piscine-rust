use std::collections::HashMap;
pub mod mall;
pub use crate::mall::*;
#[inline]
fn coerce_map<V>(m: HashMap<impl Into<String>, V>) -> HashMap<String, V> {
    m.into_iter().map(|(k, v)| (k.into(), v)).collect()
}

#[derive(Debug, Clone, PartialEq)]
pub struct Mall {
    pub name: String,
    pub guards: HashMap<String, Guard>,
    pub floors: HashMap<String, Floor>,
}

impl Mall {
    pub fn new(
        name: impl Into<String>,
        guards: HashMap<impl Into<String>, Guard>,
        floors: HashMap<impl Into<String>, Floor>,
    ) -> Self {
        Self {
            name: name.into(),
            guards: coerce_map(guards),
            floors: coerce_map(floors),
        }
    }

    pub fn change_name(&mut self, new_name: impl Into<String>) {
        self.name = new_name.into();
    }

    pub fn hire_guard(&mut self, name: impl Into<String>, guard: Guard) {
        self.guards.insert(name.into(), guard);
    }

    pub fn fire_guard(&mut self, name: impl Into<String>) {
        self.guards.remove(&name.into());
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Guard {
    pub age: u32,
    pub years_experience: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Floor {
    pub stores: HashMap<String, Store>,
    pub size_limit: u64,
}

impl Floor {
    pub fn new(stores: HashMap<impl Into<String>, Store>, size_limit: u64) -> Self {
        Self {
            stores: coerce_map(stores),
            size_limit,
        }
    }

    pub fn replace_store(&mut self, store: impl Into<String>, with: Store) {
        self.stores.entry(store.into()).and_modify(|v| *v = with);
    }

    pub fn add_store(&mut self, name: impl Into<String>, store: Store) -> Result<(), ()> {
        let has_space = self.size_limit
            >= self.stores.values().map(|s| s.square_meters).sum::<u64>() + store.square_meters;

        if has_space {
            self.stores.insert(name.into(), store);
            Ok(())
        } else {
            Err(())
        }
    }

    pub fn remove_store(&mut self, name: impl Into<String>) {
        self.stores.remove(&name.into());
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Store {
    pub employees: HashMap<String, Employee>,
    pub square_meters: u64,
}

impl Store {
    pub fn new(employees: HashMap<impl Into<String>, Employee>, square_meters: u64) -> Self {
        Self {
            employees: coerce_map(employees),
            square_meters,
        }
    }

    pub fn hire_employee(&mut self, name: impl Into<String>, employee: Employee) {
        self.employees.insert(name.into(), employee);
    }

    pub fn fire_employee(&mut self, name: impl Into<String>) {
        self.employees.remove(&name.into());
    }

    pub fn expand(&mut self, by: u64) {
        self.square_meters += by;
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Employee {
    pub age: u32,
    // The employee works from `working_hours.0` to `working_hours.1`
    pub working_hours: (u32, u32),
    pub salary: f64,
}

impl Employee {
    pub fn birthday(&mut self) {
        self.age += 1;
    }

    pub fn change_workload(&mut self, from: u32, to: u32) {
        self.working_hours = (from, to);
    }

    pub fn raise(&mut self, amount: f64) {
        self.salary += amount;
    }

    pub fn cut(&mut self, amount: f64) {
        self.salary -= amount;
    }
}

pub fn biggest_store(mall: &Mall) -> (String, Store) {
    let mut max_sq_meters: u64 = 0;
    let mut name_store = String::new();
    let mut name_floor = String::new();
    for (name, store) in &mall.floors {
        for (name_s, info) in store.stores.iter() {
            if max_sq_meters < info.square_meters {
                max_sq_meters = info.square_meters;
                name_store = name_s.clone();
                name_floor = name.clone();
            }
        }
    }
    let store = mall
        .floors
        .get(&name_floor)
        .unwrap()
        .stores
        .get(&name_store)
        .unwrap();
    (name_store, store.clone())
}
pub fn highest_paid_employee(mall: &Mall) -> Vec<(String, Employee)> {
    let mut sallry: f64 = 0.0;
    let mut name_agent: String = String::new();
    let mut name_store = String::new();
    let mut name_floor = String::new();
    let mut res: Vec<(String, Employee)> = vec![];

    for (name, store) in &mall.floors {
        for (name_s, info) in store.stores.iter() {
            for (name_emp, emp) in info.employees.iter() {
                if sallry < emp.salary {
                    sallry = emp.salary;
                    name_agent = name_emp.clone();
                    name_store = name_s.clone();
                    name_floor = name.clone();
                }
            }
        }
    }
    let emp = mall
        .floors
        .get(&name_floor)
        .unwrap()
        .stores
        .get(&name_store)
        .unwrap()
        .employees
        .get(&name_agent)
        .unwrap();
    res.push((name_agent, *emp));
    res
}

pub fn nbr_of_employees(mall: &Mall) -> usize {
    let mut nb_person: usize = 0;
    for (_, store) in &mall.floors {
        for (_, info) in store.stores.iter() {
            for (_, _) in info.employees.iter() {
                nb_person += 1;
            }
        }
    }
    for (_, _) in &mall.guards {
        nb_person += 1;
    }
    nb_person
}

pub fn check_for_securities(mall: &mut Mall, guards: HashMap<String, Guard>) {
    let mut n_guard = mall.guards.len();
    let mut total_area: u64 = 0;
    for (_, store) in &mall.floors {
        total_area += store.size_limit;
    }
    let number_quards: u64 = total_area / 200;

    for (guard_name, info) in guards {
        mall.guards.insert(guard_name.clone(), info);
        n_guard += 1;
        if n_guard == number_quards.try_into().unwrap() {
            break;
        }
    }
}

pub fn cut_or_raise(mall: &mut Mall) {
    for (_, store) in mall.floors.iter_mut() {
        for (_, info) in store.stores.clone() {
            for (_, mut emp) in info.employees {
                let working_hours = emp.working_hours.0 + emp.working_hours.1;
                let ten_percente = (emp.salary * 10.0) / 100.0;

                if (working_hours / 2) >= 10 {
                    emp.raise(ten_percente);
                } else {
                    emp.cut(ten_percente);
                }
            }
        }
    }
}
