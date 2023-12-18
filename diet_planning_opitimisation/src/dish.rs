#[derive(Debug, Clone)]
pub struct Dish {
    pub name: String,
    pub calories: u32,
    pub price: u32,
    pub carbs: u32,
    pub fats: u32,
    pub proteins: u32,
}

pub fn get_dishes() -> Vec<Dish> {
// a list of 15 dishes with polish zloty * 10 as price
    vec![
        Dish {
            name: "potatoes, 100g".into(),
            calories: 66,
            price: 2,
            carbs: 15,
            fats: 0,
            proteins: 2,
        },
        Dish {
            name: "brown rice, 100g".into(),
            calories: 111,
            price: 5,
            carbs: 23,
            fats: 1,
            proteins: 2,
        },
        Dish {
            name: "chicken breast, 100g".into(),
            calories: 165,
            price: 35,
            carbs: 0,
            fats: 3,
            proteins: 31,
        },
        Dish {
            name: "salmon fillet, 100g".into(),
            calories: 206,
            price: 65,
            carbs: 0,
            fats: 13,
            proteins: 22,
        },
        Dish {
            name: "spinach, 100g".into(),
            calories: 23,
            price: 5,
            carbs: 3,
            fats: 0,
            proteins: 3,
        },
        Dish {
            name: "broccoli, 100g".into(),
            calories: 34,
            price: 5,
            carbs: 7,
            fats: 0,
            proteins: 3,
        },
        Dish {
            name: "carrots, 100g".into(),
            calories: 41,
            price: 3,
            carbs: 10,
            fats: 0,
            proteins: 1,
        },
        Dish {
            name: "sweet potato, 100g".into(),
            calories: 86,
            price: 4,
            carbs: 20,
            fats: 0,
            proteins: 1,
        },
        Dish {
            name: "oatmeal, per serving (40g)".into(),
            calories: 68,
            price: 4,
            carbs: 12,
            fats: 1,
            proteins: 2,
        },
        Dish {
            name: "eggs, per piece".into(),
            calories: 78,
            price: 3,
            carbs: 0,
            fats: 5,
            proteins: 6,
        },
        Dish {
            name: "cottage cheese, per serving (200g)".into(),
            calories: 232,
            price: 25,
            carbs: 8,
            fats: 10,
            proteins: 28,
        },
        Dish {
            name: "yogurt, per serving (200g)".into(),
            calories: 120,
            price: 12,
            carbs: 16,
            fats: 2,
            proteins: 6,
        },
        Dish {
            name: "tuna can, drained (150g)".into(),
            calories: 150,
            price: 45,
            carbs: 0,
            fats: 2,
            proteins: 33,
        },
        Dish {
            name: "chickpeas can, drained (240g)".into(),
            calories: 360,
            price: 25,
            carbs: 60,
            fats: 6,
            proteins: 20,
        },
        Dish {
            name: "apple".into(),
            calories: 52,
            price: 12,
            carbs: 14,
            fats: 0,
            proteins: 0,
        },
    ]
}