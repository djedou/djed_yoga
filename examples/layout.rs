#[macro_use]
extern crate djed_yoga;

use djed_yoga::prelude::*;
use djed_yoga::Node;
use djed_yoga::StyleUnit::{Auto, UndefinedValue};

fn main() {
	let mut node = Node::new();

	let mut child = Node::new();
	let mut other_child = Node::new();

	node.insert_child(&mut child, 0);
	node.insert_child(&mut other_child, 1);

	style!(node,
		Margin(10 pt),
		MarginLeft(Auto),
		PaddingHorizontal(4 pt),
		Left(16 %),
		Bottom(UndefinedValue)
	);

	let child_styles = make_styles!(
		Width(32 pt),
		Height(32 pt),
		FlexGrow(1.0),
		Margin(Auto)
	);

	child.apply_styles(&child_styles);
	other_child.apply_styles(&child_styles);

	node.calculate_layout(512.0, 512.0, djed_yoga::Direction::LTR);

	println!("Layout is {:#?}", child.get_layout());
}
