// Трейт Action с методом say оформляющий приветствие
pub trait Action {
    fn person_name(&self) -> String;

    fn say(&self) -> String {
        println!("Hello, {}", self.person_name())
    }
}

// Структура Person с полем person_name в которой будет храниться выводимое имя
pub struct Person {
    person_name: String,
}

// Трейт Action для структуры Person передающий имя методу say
impl Action for Person {
    fn person_name(&self) -> String {
        format!("{}", self.person_name)
    }
}

fn main() {
    // Экземпляр структуры Person
    let person_inst = Person {
        person_name: String::from("Vlad")
    };

    // Сохранение приветствия
    let greeting = person_inst.say();

    println!("{}", greeting);
}