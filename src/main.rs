// Решение квадратного уравнения

use std::io;
use std::io::Write;
use regex::Regex;

// Вводим строку и проверяем, является ли она 
// формально правильным числом с плавающей точкой
fn input_f64() -> (f64, bool) {
	let res: f64;
	
	// Выводится приглашение с указание типа запрашиваемого числа
	print!("-f64-> ");
	match io::stdout().flush() {
		Ok(_) => {},
		Err(e) => println!("Ошибка ввода - {}", e),
	}
	
	let mut k_str: String = String::new();
	
	// Ввод строки
	io::stdin().read_line(&mut k_str)
		.ok()
		.expect("Error read line!");
	
	// Обрезаем лишние символы в началае и в конце строки
	let try_num = k_str.trim();
	
	// Регулярное выражение описывает формат числа с плавающей точкой
	let re = Regex::new(r"^(\+|-)?\d*(\.)?\d*((e|E)(\+|-)?\d+)?$").unwrap();
	
	if re.is_match(&try_num) { // Если строка соответствует заданному формату
		// то преобразуем ее в число и устанавливаем флаг корректного ввода 
		res = try_num.parse::<f64>().unwrap();
		return (res, true)
	} else {
		// иначе возвращаем ноль и флаг ошибки
		return (0.0, false)
	};
}

fn get_f64(ask: String) -> f64 {
	// Выводим пояснение о вводимых данных и добиваемся корректного ввода
	println!("{}", ask);
	
	let (mut num, mut correct) = input_f64();
	while !correct { // Пока данные не соответствуют требуемому формату
		println!("Неправильный формат ввода");
		(num, correct) = input_f64() // запрашиваем повторный ввод
	}
	num // Возвращаем правильное вещественное число
}

fn main() {
    println!("Решение квадратного уравнения");
    
    let a = get_f64("Коэффициент при x^2".to_string());
    let b = get_f64("  Коэффициент при x".to_string());
    let c = get_f64("     Свободный член".to_string());
    	
	let d: f64 = b * b - 4.0 * a * c;
	
	let b_sign: char;
	if b >= 0.0 {
		b_sign = '+'
	} else {
		b_sign = '-'
	}
	
	let c_sign: char;
	if c >= 0.0 {
		c_sign = '+'
	} else {
		c_sign = '-'
	}
	
	let b_abs =b.abs();
	let c_abs =c.abs();
	println!("\n\nРешаем уравнение {a}*x^2 {b_sign} {b_abs}*x {c_sign} {c_abs}");
	
	if d > 0.0 {
		let x1 = (-b + d.sqrt()) / 2.0 / a;
		let x2 = (-b - d.sqrt()) / 2.0 / a;
		println!("Дискриминант равен {d} > 0.0  Уравнение имеет два корня:");
		println!("x1 = {x1} \nx2 = {x2}");
	}
	
	if d == 0.0{
		let x = -b / 2.0 / a;
		println!("Дискриминант равен {d} Уравнение имеет один корень:");
		println!("x = {x}");
	}
	
	if d < 0.0 {
		let re_x = -b / 2.0 / a;
		let im_x1 = (-d).sqrt();
		let im_x2 = -(-d).sqrt();
		println!("Дискриминант равен {d} < 0.0\nУравнение имеет два сопряженных комплексных корня");
		println!("x1 = [{re_x}\t{im_x1}]");
		println!("x2 = [{re_x}\t{im_x2}]");
	}
}
