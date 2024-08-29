// Трейт Action с методом say выводящий приветствие
pub trait Action {
    fn say(&self);
}

// Структура Person с полем person_name в которой будет храниться выводимое имя
pub struct Person {
    person_name: String,
}

// Трейт Action для структуры Person
impl Action for Person {
    fn say(&self) {
        println!("Hello, {}", self.person_name);
    }
}

fn main() {
    // Экземпляр структуры Person
    let person_inst = Person {
        person_name: String::from("Vlad")
    };

    // Вывод приветствия
    person_inst.say();
}
