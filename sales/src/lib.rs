#[derive(Debug, Clone, PartialEq)]
pub struct Store {
    pub products: Vec<(String, f32)>,
}
impl Store {
    pub fn new(products: Vec<(String, f32)>) -> Store {
        Store { products }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Cart {
    pub items: Vec<(String, f32)>,
    pub receipt: Vec<f32>,
}
impl Cart {
    pub fn new() -> Cart {
        Cart {
            items: vec![],
            receipt: vec![],
        }
    }
    pub fn insert_item(&mut self, s: &Store, ele: String) {
        let get_product = || -> (String, f32) {
            s.products
                .iter()
                .filter(|(name, _)| name.to_string() == ele)
                .cloned()
                .collect::<Vec<(String, f32)>>()
                .into_iter()
                .next()
                .unwrap()
        };
        self.items.push(get_product());
    }
    pub fn generate_receipt(&mut self) -> Vec<f32> {
        let mut prices = self.items.iter().map(|(_, p)| *p).collect::<Vec<f32>>();
        prices.sort_by(|a, b| a.partial_cmp(&b).unwrap());
        let discount = prices.len() / 3;
        if discount <= 0 {
            return prices;
        }
        let per = prices[..discount].iter().sum::<f32>() / prices.iter().sum::<f32>();
        let mut res: Vec<f32> = vec![];
        for ele in prices {
            let new_price = ele - (ele * per);
            let formated=format!("{:.2}",new_price);
            res.push(formated.parse().unwrap());
        }
        self.receipt = res.clone();
        res
    }
}
