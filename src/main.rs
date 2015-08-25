#![feature(convert, test, core, core_str_ext)]
extern crate test;
extern crate core;

use core::str::StrExt;

#[derive(Clone)]
pub struct Entity {
    start: usize,
	end: usize,
    html: String
}

fn render(text: &str, entities: &mut Vec<Entity>) -> String {
	let mut sb = String::new();
	entities.sort_by(|e1, e2| e1.start.cmp(&e2.start) );

  let mut pos = 0 as usize;
	for entity in entities {
		sb.push_str(text.slice_chars(pos, entity.start));
		sb.push_str(entity.html.as_str());
		pos = entity.end;
	}
	sb.push_str(text.slice_chars(pos, text.len()));
	sb
}

fn main() {
	let result = classic(&mut entities());
	println!("Result: {}", result);
}

pub fn classic(entities: &mut Vec<Entity>) -> String {
    render("Attend to hear 6 stellar #mobile #startups at #OF12 Entrepreneur Idol show 2day,  http://t.co/HtzEMgAC @TiEcon @sv_entrepreneur @500!", entities)
}

pub fn entities() -> Vec<Entity> {
	let entities = vec![
	Entity {start: 82, end: 102, html:"<http://t.co/HtzEMgAC>".to_string()},
	Entity {start: 128, end: 132, html:"<@500>".to_string()},
	Entity {start: 25, end: 32, html:"<#mobile>".to_string()},
	Entity {start: 33, end: 42, html:"<#startups>".to_string()},
	Entity {start: 111, end: 127, html:"<@sv_entrepreneur>".to_string()},
	Entity {start: 46, end: 51, html:"<#OF12>".to_string()},
	Entity {start: 103, end: 110, html:"<@TiEcon>".to_string()},
	];
	entities
}

#[cfg(test)]
mod rendertest {
    use super::*;
    use test::Bencher;

    #[test]
	fn correctness() {
		let result = "Attend to hear 6 stellar <#mobile> <#startups> at <#OF12> Entrepreneur Idol show 2day,  <http://t.co/HtzEMgAC> <@TiEcon> <@sv_entrepreneur> <@500>!";
		assert_eq!(result, classic(&mut entities()))
	}

    #[bench]
	fn bench_replacement(b: &mut Bencher) {
	   	let entities = &mut entities();
		b.iter(|| {
		    classic(entities)
		});
	}
}
