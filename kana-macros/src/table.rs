use proc_macro2::Ident;
use syn::{
	braced,
	ext::IdentExt,
	parse::{discouraged::Speculative, Parse, ParseStream},
	Error, LitChar, LitStr, Result, Token,
};

#[derive(Debug)]
pub struct TableSet(pub Vec<Table>);

#[derive(Debug)]
pub struct Table {
	pub name: String,
	pub rows: Vec<TableRow>,
}

#[derive(Debug)]
pub enum TableRow {
	Map { from: String, to: String },
}

impl Parse for TableSet {
	fn parse(input: ParseStream) -> Result<Self> {
		let mut tables: Vec<Table> = Vec::new();

		loop {
			let next = input.fork();
			if let Ok(name) = parse_ident(&next) {
				input.advance_to(&next);
				input.parse::<Token![=>]>()?;

				let content;
				braced!(content in input);

				let rows = content.parse_terminated::<_, Token![,]>(TableRow::parse)?;
				let mut rows = rows.into_iter().collect::<Vec<_>>();

				let item = tables.iter_mut().find(|x| x.name == name);
				if let Some(item) = item {
					item.rows.append(&mut rows);
				} else {
					tables.push(Table { name, rows });
				}
			} else if !input.is_empty() {
				return Err(Error::new(
					next.span(),
					format!("expected a table with `identifier => {{ ... }}` grammar"),
				));
			} else {
				break;
			}
		}

		Ok(TableSet(tables))
	}
}

impl Parse for TableRow {
	fn parse(input: ParseStream) -> Result<Self> {
		let from = parse_string(input)?;
		input.parse::<Token![:]>()?;
		let to = parse_string(input)?;
		Ok(TableRow::Map { from, to })
	}
}

fn parse_ident(input: ParseStream) -> Result<String> {
	input.call(Ident::parse_any).map(|x| x.unraw().to_string())
}

fn parse_string(input: ParseStream) -> Result<String> {
	let next = input.fork();
	let fork = input.fork();
	if let Ok(char) = fork.parse::<LitChar>() {
		input.advance_to(&fork);
		Ok(char.value().to_string())
	} else {
		input
			.parse::<LitStr>()
			.map(|x| x.value())
			.map_err(|_| Error::new(next.span(), "expected a string or char literal"))
	}
}
