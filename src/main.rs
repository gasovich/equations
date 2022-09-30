// Решение квадратного уравнения

use std::io;
use std::io::Write;

fn get_f64(ask: &str) -> f64 {
	let mut buffer = String::new();
	let mut correct: bool = false;
	let mut res: f64 = 0.0;
	
	while !correct {
		// Печатаем промпт
		print!("{}", ask);
		match io::stdout().flush() {
			Ok(_) => {},
			Err(e) => println!("Ошибка вывода - {}", e),
		}
		
		// Читаем строку
		match io::stdin().read_line(&mut buffer) {
			Ok(_) => {},
			Err(e) => println!("Ошибка ввода - {}", e),
		};
		
		// Пытаемся преобразовать строку в число, получаем данные в формате Result
		let k: Result<f64, _> = buffer.trim().parse();
		match k {
			Ok(_) => {
				res = k.unwrap();
				correct = true;
			},
			Err(_) => {
				println!("Неправильный формат ввода");
				buffer = "".to_string()
			}
		}
	}
	res
}

fn main() {
    println!("Решение квадратного уравнения");
    
    let a = get_f64("Коэффициент при x^2 --> ");
    let b = get_f64("  Коэффициент при x --> ");
    let c = get_f64("     Свободный член --> ");
    	
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
	println!("\n\nРешаем уравнение {a}*x^2 {b_sign} {b_abs}*x {c_sign} {c_abs} = 0");
	
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
