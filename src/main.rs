use rand::Rng;
use std::cmp::Ordering;
use std::io; // стандартный ввод-вывод, подключаем библиотеку с помощью "use"a-ala"using" //используем библиотеку rang для генерации случайного числа
             //fn dodawanie() {
             // let x = 5;
             // let y = 10;
             // println!("x = {} and y = {}", x, y)
             //}

fn main() {
    //Main всегда является точкой входа в программе
    //    println!("Guess the number!"); //println! - МАКРОС, который выводит строку на экран

    //    println!("Please input your guess.");

    //    let mut guess = String::new(); // Переменная для хранения ввода который находтся ниже,
    // Для создания переменной мы используем оператор let
    // К примеру, с "let apples = 5;" создаст новую переменную с именем apples и привяжет к ней
    // значение 5. Стоить заметить, что переменные в Раст неизменяемые по умолчанию, если мы хотим
    // чтобы переменная была изменяемая, мы используем "let mut"
    // let apples = 5; - неизменяемая
    // let mut bananas = 5; - изменяемая
    //Sting::new - вовращает экземпляр String
    //::new - Ассоциированная функция - это функция котопая реализованная для типа, в данном случае
    //В конечном итоге строка let mut guess = String::new(); создала изменяемую переменную, которая связывается с новым пустым экземпляром String
    ////////////////////////////////////////////////////////////////////////////////////////////////
    //Получение данных от пользователя
    //    io::stdin() //stdin - входная строка от пользователя
    //        .read_line(&mut guess) //строка .read_line(&mut guess) вызывает метод read_line на дескрипторе стандартного ввода
    // Мы также передаём &mut guess в качестве аргумента read_line
    //.expect является МЕТОДОМ, мы также могли написать эти строчки так....
    //        io::stdin().read_line(&mut guess).expect("Failed to read line");
    //МЕТОД вызываем через точку "."
    //         .expect("Failed to read line"); //вывод о ошибке

    //      println!("You guessed: {guess}");

    //    let x = 5;
    //    let y = 10;
    //    println!("x = {} and y = {}", x, y)
    ///////////////////////////////////////////////////////////////////////////////////
    //    println!("Guess the number!");
    //    let secret_number = rand::thread_rng().gen_range(1..=100);
    //    println!("The secret number is: {secret_number}");
    //    println!("Please input your guess.");
    //    let mut guess = String::new();
    //    io::stdin()
    //        .read_line(&mut guess)
    //        .expect("Failed to read line");
    //    println!("You guessed: {guess}")
    ////////////////////////////////////////////////////////////////////////////////////
    let secret_code = rand::thread_rng().gen_range(1..=100);
    println!("Твой секретный код - {secret_code}");
    let mut answer = String::new();
    io::stdin().read_line(&mut answer).expect("Ошибка");

    println!("Твой ответ: {answer}");
}
