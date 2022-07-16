// Решение квадратного уравнения

use std::io;

fn main() {
    println!("Решение квадратного уравнения");
    let mut a_str = String::new();
    let mut b_str = String::new();
    let mut c_str = String::new();
    
    println!("Коэффициент при x^2");
    match io::stdin().read_line(&mut a_str) {
		Ok(_) => {},
		Err(e) => println!("Ошибка ввода - {}", e),
	}
	
	println!("  Коэффициент при x");
    match io::stdin().read_line(&mut b_str) {
		Ok(_) => {},
		Err(e) => println!("Ошибка ввода - {}", e),
	}
	
	println!("   Свободный член");
    match io::stdin().read_line(&mut c_str) {
		Ok(_) => {},
		Err(e) => println!("Ошибка ввода - {}", e),
	}
	
	let a: f64 = a_str.trim().parse().unwrap();
	let b: f64 = b_str.trim().parse().unwrap();
	let c: f64 = c_str.trim().parse().unwrap();
	
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
		println!("Дискриминант равен {d} < 0.0  Уравнение не имеет вещественных корней");
	}
}
