use json::object;

pub struct Food {
    pub name: String,
    pub calories: (String, String),
    pub proteins: f64,
    pub fats: f64,
    pub carbs: f64,
    pub nbr_of_portions: f64,
}

pub fn calculate_macros(foods: &[Food]) -> json::JsonValue {
    let mut cals: f64 = 0.0;
    let mut carbs: f64 = 0.0;
    let mut proteins: f64 = 0.0;
    let mut fats: f64 = 0.0;

    for food in foods {
        let nbr_of_portions: f64 = food.nbr_of_portions;
        let n_cals = food.calories.1.split('k').collect::<Vec<_>>();
        cals += (n_cals[0].parse::<f64>().unwrap()) * nbr_of_portions;
        carbs += food.carbs * nbr_of_portions;
        proteins += food.proteins * nbr_of_portions;
        fats += food.fats * nbr_of_portions;
    }
    object! {
       cals:(cals*100.0).round()/100.0,
        carbs: (carbs*100.0).round()/100.0,
        proteins:(proteins*100.0).round()/100.0,
        fats:(fats*100.0).round()/100.0
    }
}
