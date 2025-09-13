pub mod mall;
use std::collections::HashMap;
pub use mall::{Employee, Guard, Mall, Store,Floor};

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
        mall.hire_guard(guard_name.clone(), info);
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
