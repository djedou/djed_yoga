
extern crate ordered_float;
extern crate djed_yoga;

use djed_yoga::*;

#[test]
fn test_absolute_layout_width_height_start_top() {
	let mut config = Config::new();

	let mut root = Node::new_with_config(&mut config);
	root.set_width(StyleUnit::Point((100 as f32).into()));
	root.set_height(StyleUnit::Point((100 as f32).into()));

	let mut root_child0 = Node::new_with_config(&mut config);
	root_child0.set_position_type(PositionType::Absolute);
	root_child0.set_position(Edge::Start, StyleUnit::Point((10 as f32).into()));
	root_child0.set_position(Edge::Top, StyleUnit::Point((10 as f32).into()));
	root_child0.set_width(StyleUnit::Point((10 as f32).into()));
	root_child0.set_height(StyleUnit::Point((10 as f32).into()));
	root.insert_child(&mut root_child0, 0);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	assert_eq!(0 as f32, root.get_layout_left());
	assert_eq!(0 as f32, root.get_layout_top());
	assert_eq!(100 as f32, root.get_layout_width());
	assert_eq!(100 as f32, root.get_layout_height());

	assert_eq!(10 as f32, root_child0.get_layout_left());
	assert_eq!(10 as f32, root_child0.get_layout_top());
	assert_eq!(10 as f32, root_child0.get_layout_width());
	assert_eq!(10 as f32, root_child0.get_layout_height());

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	assert_eq!(0 as f32, root.get_layout_left());
	assert_eq!(0 as f32, root.get_layout_top());
	assert_eq!(100 as f32, root.get_layout_width());
	assert_eq!(100 as f32, root.get_layout_height());

	assert_eq!(80 as f32, root_child0.get_layout_left());
	assert_eq!(10 as f32, root_child0.get_layout_top());
	assert_eq!(10 as f32, root_child0.get_layout_width());
	assert_eq!(10 as f32, root_child0.get_layout_height());
}

#[test]
fn test_absolute_layout_width_height_end_bottom() {
	let mut config = Config::new();

	let mut root = Node::new_with_config(&mut config);
	root.set_width(StyleUnit::Point((100 as f32).into()));
	root.set_height(StyleUnit::Point((100 as f32).into()));

	let mut root_child0 = Node::new_with_config(&mut config);
	root_child0.set_position_type(PositionType::Absolute);
	root_child0.set_position(Edge::End, StyleUnit::Point((10 as f32).into()));
	root_child0.set_position(Edge::Bottom, StyleUnit::Point((10 as f32).into()));
	root_child0.set_width(StyleUnit::Point((10 as f32).into()));
	root_child0.set_height(StyleUnit::Point((10 as f32).into()));
	root.insert_child(&mut root_child0, 0);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	assert_eq!(0 as f32, root.get_layout_left());
	assert_eq!(0 as f32, root.get_layout_top());
	assert_eq!(100 as f32, root.get_layout_width());
	assert_eq!(100 as f32, root.get_layout_height());

	assert_eq!(80 as f32, root_child0.get_layout_left());
	assert_eq!(80 as f32, root_child0.get_layout_top());
	assert_eq!(10 as f32, root_child0.get_layout_width());
	assert_eq!(10 as f32, root_child0.get_layout_height());

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	assert_eq!(0 as f32, root.get_layout_left());
	assert_eq!(0 as f32, root.get_layout_top());
	assert_eq!(100 as f32, root.get_layout_width());
	assert_eq!(100 as f32, root.get_layout_height());

	assert_eq!(10 as f32, root_child0.get_layout_left());
	assert_eq!(80 as f32, root_child0.get_layout_top());
	assert_eq!(10 as f32, root_child0.get_layout_width());
	assert_eq!(10 as f32, root_child0.get_layout_height());
}

#[test]
fn test_absolute_layout_start_top_end_bottom() {
	let mut config = Config::new();

	let mut root = Node::new_with_config(&mut config);
	root.set_width(StyleUnit::Point((100 as f32).into()));
	root.set_height(StyleUnit::Point((100 as f32).into()));

	let mut root_child0 = Node::new_with_config(&mut config);
	root_child0.set_position_type(PositionType::Absolute);
	root_child0.set_position(Edge::Start, StyleUnit::Point((10 as f32).into()));
	root_child0.set_position(Edge::Top, StyleUnit::Point((10 as f32).into()));
	root_child0.set_position(Edge::End, StyleUnit::Point((10 as f32).into()));
	root_child0.set_position(Edge::Bottom, StyleUnit::Point((10 as f32).into()));
	root.insert_child(&mut root_child0, 0);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	assert_eq!(0 as f32, root.get_layout_left());
	assert_eq!(0 as f32, root.get_layout_top());
	assert_eq!(100 as f32, root.get_layout_width());
	assert_eq!(100 as f32, root.get_layout_height());

	assert_eq!(10 as f32, root_child0.get_layout_left());
	assert_eq!(10 as f32, root_child0.get_layout_top());
	assert_eq!(80 as f32, root_child0.get_layout_width());
	assert_eq!(80 as f32, root_child0.get_layout_height());

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	assert_eq!(0 as f32, root.get_layout_left());
	assert_eq!(0 as f32, root.get_layout_top());
	assert_eq!(100 as f32, root.get_layout_width());
	assert_eq!(100 as f32, root.get_layout_height());

	assert_eq!(10 as f32, root_child0.get_layout_left());
	assert_eq!(10 as f32, root_child0.get_layout_top());
	assert_eq!(80 as f32, root_child0.get_layout_width());
	assert_eq!(80 as f32, root_child0.get_layout_height());
}

#[test]
fn test_absolute_layout_width_height_start_top_end_bottom() {
	let mut config = Config::new();

	let mut root = Node::new_with_config(&mut config);
	root.set_width(StyleUnit::Point((100 as f32).into()));
	root.set_height(StyleUnit::Point((100 as f32).into()));

	let mut root_child0 = Node::new_with_config(&mut config);
	root_child0.set_position_type(PositionType::Absolute);
	root_child0.set_position(Edge::Start, StyleUnit::Point((10 as f32).into()));
	root_child0.set_position(Edge::Top, StyleUnit::Point((10 as f32).into()));
	root_child0.set_position(Edge::End, StyleUnit::Point((10 as f32).into()));
	root_child0.set_position(Edge::Bottom, StyleUnit::Point((10 as f32).into()));
	root_child0.set_width(StyleUnit::Point((10 as f32).into()));
	root_child0.set_height(StyleUnit::Point((10 as f32).into()));
	root.insert_child(&mut root_child0, 0);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	assert_eq!(0 as f32, root.get_layout_left());
	assert_eq!(0 as f32, root.get_layout_top());
	assert_eq!(100 as f32, root.get_layout_width());
	assert_eq!(100 as f32, root.get_layout_height());

	assert_eq!(10 as f32, root_child0.get_layout_left());
	assert_eq!(10 as f32, root_child0.get_layout_top());
	assert_eq!(10 as f32, root_child0.get_layout_width());
	assert_eq!(10 as f32, root_child0.get_layout_height());

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	assert_eq!(0 as f32, root.get_layout_left());
	assert_eq!(0 as f32, root.get_layout_top());
	assert_eq!(100 as f32, root.get_layout_width());
	assert_eq!(100 as f32, root.get_layout_height());

	assert_eq!(80 as f32, root_child0.get_layout_left());
	assert_eq!(10 as f32, root_child0.get_layout_top());
	assert_eq!(10 as f32, root_child0.get_layout_width());
	assert_eq!(10 as f32, root_child0.get_layout_height());
}

#[test]
fn test_do_not_clamp_height_of_absolute_node_to_height_of_its_overflow_hidden_parent() {
	let mut config = Config::new();

	let mut root = Node::new_with_config(&mut config);
	root.set_flex_direction(FlexDirection::Row);
	root.set_overflow(Overflow::Hidden);
	root.set_width(StyleUnit::Point((50 as f32).into()));
	root.set_height(StyleUnit::Point((50 as f32).into()));

	let mut root_child0 = Node::new_with_config(&mut config);
	root_child0.set_position_type(PositionType::Absolute);
	root_child0.set_position(Edge::Start, StyleUnit::Point((0 as f32).into()));
	root_child0.set_position(Edge::Top, StyleUnit::Point((0 as f32).into()));
	root.insert_child(&mut root_child0, 0);

	let mut root_child0_child0 = Node::new_with_config(&mut config);
	root_child0_child0.set_width(StyleUnit::Point((100 as f32).into()));
	root_child0_child0.set_min_width(StyleUnit::Auto);
	root_child0_child0.set_height(StyleUnit::Point((100 as f32).into()));
	root_child0_child0.set_min_height(StyleUnit::Auto);
	root_child0.insert_child(&mut root_child0_child0, 0);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	assert_eq!(0 as f32, root.get_layout_left());
	assert_eq!(0 as f32, root.get_layout_top());
	assert_eq!(50 as f32, root.get_layout_width());
	assert_eq!(50 as f32, root.get_layout_height());

	assert_eq!(0 as f32, root_child0.get_layout_left());
	assert_eq!(0 as f32, root_child0.get_layout_top());
	assert_eq!(100 as f32, root_child0.get_layout_width());
	assert_eq!(100 as f32, root_child0.get_layout_height());

	assert_eq!(0 as f32, root_child0_child0.get_layout_left());
	assert_eq!(0 as f32, root_child0_child0.get_layout_top());
	assert_eq!(100 as f32, root_child0_child0.get_layout_width());
	assert_eq!(100 as f32, root_child0_child0.get_layout_height());

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	assert_eq!(0 as f32, root.get_layout_left());
	assert_eq!(0 as f32, root.get_layout_top());
	assert_eq!(50 as f32, root.get_layout_width());
	assert_eq!(50 as f32, root.get_layout_height());

	assert_eq!(-50 as f32, root_child0.get_layout_left());
	assert_eq!(0 as f32, root_child0.get_layout_top());
	assert_eq!(100 as f32, root_child0.get_layout_width());
	assert_eq!(100 as f32, root_child0.get_layout_height());

	assert_eq!(0 as f32, root_child0_child0.get_layout_left());
	assert_eq!(0 as f32, root_child0_child0.get_layout_top());
	assert_eq!(100 as f32, root_child0_child0.get_layout_width());
	assert_eq!(100 as f32, root_child0_child0.get_layout_height());
}

#[test]
fn test_absolute_layout_within_border() {
	let mut config = Config::new();

	let mut root = Node::new_with_config(&mut config);
	root.set_margin(Edge::Left, StyleUnit::Point((10 as f32).into()));
	root.set_margin(Edge::Top, StyleUnit::Point((10 as f32).into()));
	root.set_margin(Edge::Right, StyleUnit::Point((10 as f32).into()));
	root.set_margin(Edge::Bottom, StyleUnit::Point((10 as f32).into()));
	root.set_padding(Edge::Left, StyleUnit::Point((10 as f32).into()));
	root.set_padding(Edge::Top, StyleUnit::Point((10 as f32).into()));
	root.set_padding(Edge::Right, StyleUnit::Point((10 as f32).into()));
	root.set_padding(Edge::Bottom, StyleUnit::Point((10 as f32).into()));
	root.set_border(Edge::Left, 10 as f32);
	root.set_border(Edge::Top, 10 as f32);
	root.set_border(Edge::Right, 10 as f32);
	root.set_border(Edge::Bottom, 10 as f32);
	root.set_width(StyleUnit::Point((100 as f32).into()));
	root.set_height(StyleUnit::Point((100 as f32).into()));

	let mut root_child0 = Node::new_with_config(&mut config);
	root_child0.set_position_type(PositionType::Absolute);
	root_child0.set_position(Edge::Left, StyleUnit::Point((0 as f32).into()));
	root_child0.set_position(Edge::Top, StyleUnit::Point((0 as f32).into()));
	root_child0.set_width(StyleUnit::Point((50 as f32).into()));
	root_child0.set_height(StyleUnit::Point((50 as f32).into()));
	root.insert_child(&mut root_child0, 0);

	let mut root_child1 = Node::new_with_config(&mut config);
	root_child1.set_position_type(PositionType::Absolute);
	root_child1.set_position(Edge::Right, StyleUnit::Point((0 as f32).into()));
	root_child1.set_position(Edge::Bottom, StyleUnit::Point((0 as f32).into()));
	root_child1.set_width(StyleUnit::Point((50 as f32).into()));
	root_child1.set_height(StyleUnit::Point((50 as f32).into()));
	root.insert_child(&mut root_child1, 1);

	let mut root_child2 = Node::new_with_config(&mut config);
	root_child2.set_position_type(PositionType::Absolute);
	root_child2.set_position(Edge::Left, StyleUnit::Point((0 as f32).into()));
	root_child2.set_position(Edge::Top, StyleUnit::Point((0 as f32).into()));
	root_child2.set_margin(Edge::Left, StyleUnit::Point((10 as f32).into()));
	root_child2.set_margin(Edge::Top, StyleUnit::Point((10 as f32).into()));
	root_child2.set_margin(Edge::Right, StyleUnit::Point((10 as f32).into()));
	root_child2.set_margin(Edge::Bottom, StyleUnit::Point((10 as f32).into()));
	root_child2.set_width(StyleUnit::Point((50 as f32).into()));
	root_child2.set_height(StyleUnit::Point((50 as f32).into()));
	root.insert_child(&mut root_child2, 2);

	let mut root_child3 = Node::new_with_config(&mut config);
	root_child3.set_position_type(PositionType::Absolute);
	root_child3.set_position(Edge::Right, StyleUnit::Point((0 as f32).into()));
	root_child3.set_position(Edge::Bottom, StyleUnit::Point((0 as f32).into()));
	root_child3.set_margin(Edge::Left, StyleUnit::Point((10 as f32).into()));
	root_child3.set_margin(Edge::Top, StyleUnit::Point((10 as f32).into()));
	root_child3.set_margin(Edge::Right, StyleUnit::Point((10 as f32).into()));
	root_child3.set_margin(Edge::Bottom, StyleUnit::Point((10 as f32).into()));
	root_child3.set_width(StyleUnit::Point((50 as f32).into()));
	root_child3.set_height(StyleUnit::Point((50 as f32).into()));
	root.insert_child(&mut root_child3, 3);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	assert_eq!(10 as f32, root.get_layout_left());
	assert_eq!(10 as f32, root.get_layout_top());
	assert_eq!(100 as f32, root.get_layout_width());
	assert_eq!(100 as f32, root.get_layout_height());

	assert_eq!(10 as f32, root_child0.get_layout_left());
	assert_eq!(10 as f32, root_child0.get_layout_top());
	assert_eq!(50 as f32, root_child0.get_layout_width());
	assert_eq!(50 as f32, root_child0.get_layout_height());

	assert_eq!(40 as f32, root_child1.get_layout_left());
	assert_eq!(40 as f32, root_child1.get_layout_top());
	assert_eq!(50 as f32, root_child1.get_layout_width());
	assert_eq!(50 as f32, root_child1.get_layout_height());

	assert_eq!(20 as f32, root_child2.get_layout_left());
	assert_eq!(20 as f32, root_child2.get_layout_top());
	assert_eq!(50 as f32, root_child2.get_layout_width());
	assert_eq!(50 as f32, root_child2.get_layout_height());

	assert_eq!(30 as f32, root_child3.get_layout_left());
	assert_eq!(30 as f32, root_child3.get_layout_top());
	assert_eq!(50 as f32, root_child3.get_layout_width());
	assert_eq!(50 as f32, root_child3.get_layout_height());

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	assert_eq!(10 as f32, root.get_layout_left());
	assert_eq!(10 as f32, root.get_layout_top());
	assert_eq!(100 as f32, root.get_layout_width());
	assert_eq!(100 as f32, root.get_layout_height());

	assert_eq!(10 as f32, root_child0.get_layout_left());
	assert_eq!(10 as f32, root_child0.get_layout_top());
	assert_eq!(50 as f32, root_child0.get_layout_width());
	assert_eq!(50 as f32, root_child0.get_layout_height());

	assert_eq!(40 as f32, root_child1.get_layout_left());
	assert_eq!(40 as f32, root_child1.get_layout_top());
	assert_eq!(50 as f32, root_child1.get_layout_width());
	assert_eq!(50 as f32, root_child1.get_layout_height());

	assert_eq!(20 as f32, root_child2.get_layout_left());
	assert_eq!(20 as f32, root_child2.get_layout_top());
	assert_eq!(50 as f32, root_child2.get_layout_width());
	assert_eq!(50 as f32, root_child2.get_layout_height());

	assert_eq!(30 as f32, root_child3.get_layout_left());
	assert_eq!(30 as f32, root_child3.get_layout_top());
	assert_eq!(50 as f32, root_child3.get_layout_width());
	assert_eq!(50 as f32, root_child3.get_layout_height());
}

#[test]
fn test_absolute_layout_align_items_and_justify_content_center() {
	let mut config = Config::new();

	let mut root = Node::new_with_config(&mut config);
	root.set_justify_content(Justify::Center);
	root.set_align_items(Align::Center);
	root.set_flex_grow(1 as f32);
	root.set_width(StyleUnit::Point((110 as f32).into()));
	root.set_height(StyleUnit::Point((100 as f32).into()));

	let mut root_child0 = Node::new_with_config(&mut config);
	root_child0.set_position_type(PositionType::Absolute);
	root_child0.set_width(StyleUnit::Point((60 as f32).into()));
	root_child0.set_height(StyleUnit::Point((40 as f32).into()));
	root.insert_child(&mut root_child0, 0);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	assert_eq!(0 as f32, root.get_layout_left());
	assert_eq!(0 as f32, root.get_layout_top());
	assert_eq!(110 as f32, root.get_layout_width());
	assert_eq!(100 as f32, root.get_layout_height());

	assert_eq!(25 as f32, root_child0.get_layout_left());
	assert_eq!(30 as f32, root_child0.get_layout_top());
	assert_eq!(60 as f32, root_child0.get_layout_width());
	assert_eq!(40 as f32, root_child0.get_layout_height());

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	assert_eq!(0 as f32, root.get_layout_left());
	assert_eq!(0 as f32, root.get_layout_top());
	assert_eq!(110 as f32, root.get_layout_width());
	assert_eq!(100 as f32, root.get_layout_height());

	assert_eq!(25 as f32, root_child0.get_layout_left());
	assert_eq!(30 as f32, root_child0.get_layout_top());
	assert_eq!(60 as f32, root_child0.get_layout_width());
	assert_eq!(40 as f32, root_child0.get_layout_height());
}

#[test]
fn test_absolute_layout_align_items_and_justify_content_flex_end() {
	let mut config = Config::new();

	let mut root = Node::new_with_config(&mut config);
	root.set_justify_content(Justify::FlexEnd);
	root.set_align_items(Align::FlexEnd);
	root.set_flex_grow(1 as f32);
	root.set_width(StyleUnit::Point((110 as f32).into()));
	root.set_height(StyleUnit::Point((100 as f32).into()));

	let mut root_child0 = Node::new_with_config(&mut config);
	root_child0.set_position_type(PositionType::Absolute);
	root_child0.set_width(StyleUnit::Point((60 as f32).into()));
	root_child0.set_height(StyleUnit::Point((40 as f32).into()));
	root.insert_child(&mut root_child0, 0);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	assert_eq!(0 as f32, root.get_layout_left());
	assert_eq!(0 as f32, root.get_layout_top());
	assert_eq!(110 as f32, root.get_layout_width());
	assert_eq!(100 as f32, root.get_layout_height());

	assert_eq!(50 as f32, root_child0.get_layout_left());
	assert_eq!(60 as f32, root_child0.get_layout_top());
	assert_eq!(60 as f32, root_child0.get_layout_width());
	assert_eq!(40 as f32, root_child0.get_layout_height());

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	assert_eq!(0 as f32, root.get_layout_left());
	assert_eq!(0 as f32, root.get_layout_top());
	assert_eq!(110 as f32, root.get_layout_width());
	assert_eq!(100 as f32, root.get_layout_height());

	assert_eq!(0 as f32, root_child0.get_layout_left());
	assert_eq!(60 as f32, root_child0.get_layout_top());
	assert_eq!(60 as f32, root_child0.get_layout_width());
	assert_eq!(40 as f32, root_child0.get_layout_height());
}

#[test]
fn test_absolute_layout_justify_content_center() {
	let mut config = Config::new();

	let mut root = Node::new_with_config(&mut config);
	root.set_justify_content(Justify::Center);
	root.set_flex_grow(1 as f32);
	root.set_width(StyleUnit::Point((110 as f32).into()));
	root.set_height(StyleUnit::Point((100 as f32).into()));

	let mut root_child0 = Node::new_with_config(&mut config);
	root_child0.set_position_type(PositionType::Absolute);
	root_child0.set_width(StyleUnit::Point((60 as f32).into()));
	root_child0.set_height(StyleUnit::Point((40 as f32).into()));
	root.insert_child(&mut root_child0, 0);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	assert_eq!(0 as f32, root.get_layout_left());
	assert_eq!(0 as f32, root.get_layout_top());
	assert_eq!(110 as f32, root.get_layout_width());
	assert_eq!(100 as f32, root.get_layout_height());

	assert_eq!(0 as f32, root_child0.get_layout_left());
	assert_eq!(30 as f32, root_child0.get_layout_top());
	assert_eq!(60 as f32, root_child0.get_layout_width());
	assert_eq!(40 as f32, root_child0.get_layout_height());

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	assert_eq!(0 as f32, root.get_layout_left());
	assert_eq!(0 as f32, root.get_layout_top());
	assert_eq!(110 as f32, root.get_layout_width());
	assert_eq!(100 as f32, root.get_layout_height());

	assert_eq!(50 as f32, root_child0.get_layout_left());
	assert_eq!(30 as f32, root_child0.get_layout_top());
	assert_eq!(60 as f32, root_child0.get_layout_width());
	assert_eq!(40 as f32, root_child0.get_layout_height());
}

#[test]
fn test_absolute_layout_align_items_center() {
	let mut config = Config::new();

	let mut root = Node::new_with_config(&mut config);
	root.set_align_items(Align::Center);
	root.set_flex_grow(1 as f32);
	root.set_width(StyleUnit::Point((110 as f32).into()));
	root.set_height(StyleUnit::Point((100 as f32).into()));

	let mut root_child0 = Node::new_with_config(&mut config);
	root_child0.set_position_type(PositionType::Absolute);
	root_child0.set_width(StyleUnit::Point((60 as f32).into()));
	root_child0.set_height(StyleUnit::Point((40 as f32).into()));
	root.insert_child(&mut root_child0, 0);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	assert_eq!(0 as f32, root.get_layout_left());
	assert_eq!(0 as f32, root.get_layout_top());
	assert_eq!(110 as f32, root.get_layout_width());
	assert_eq!(100 as f32, root.get_layout_height());

	assert_eq!(25 as f32, root_child0.get_layout_left());
	assert_eq!(0 as f32, root_child0.get_layout_top());
	assert_eq!(60 as f32, root_child0.get_layout_width());
	assert_eq!(40 as f32, root_child0.get_layout_height());

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	assert_eq!(0 as f32, root.get_layout_left());
	assert_eq!(0 as f32, root.get_layout_top());
	assert_eq!(110 as f32, root.get_layout_width());
	assert_eq!(100 as f32, root.get_layout_height());

	assert_eq!(25 as f32, root_child0.get_layout_left());
	assert_eq!(0 as f32, root_child0.get_layout_top());
	assert_eq!(60 as f32, root_child0.get_layout_width());
	assert_eq!(40 as f32, root_child0.get_layout_height());
}

#[test]
fn test_absolute_layout_align_items_center_on_child_only() {
	let mut config = Config::new();

	let mut root = Node::new_with_config(&mut config);
	root.set_flex_grow(1 as f32);
	root.set_width(StyleUnit::Point((110 as f32).into()));
	root.set_height(StyleUnit::Point((100 as f32).into()));

	let mut root_child0 = Node::new_with_config(&mut config);
	root_child0.set_align_self(Align::Center);
	root_child0.set_position_type(PositionType::Absolute);
	root_child0.set_width(StyleUnit::Point((60 as f32).into()));
	root_child0.set_height(StyleUnit::Point((40 as f32).into()));
	root.insert_child(&mut root_child0, 0);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	assert_eq!(0 as f32, root.get_layout_left());
	assert_eq!(0 as f32, root.get_layout_top());
	assert_eq!(110 as f32, root.get_layout_width());
	assert_eq!(100 as f32, root.get_layout_height());

	assert_eq!(25 as f32, root_child0.get_layout_left());
	assert_eq!(0 as f32, root_child0.get_layout_top());
	assert_eq!(60 as f32, root_child0.get_layout_width());
	assert_eq!(40 as f32, root_child0.get_layout_height());

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	assert_eq!(0 as f32, root.get_layout_left());
	assert_eq!(0 as f32, root.get_layout_top());
	assert_eq!(110 as f32, root.get_layout_width());
	assert_eq!(100 as f32, root.get_layout_height());

	assert_eq!(25 as f32, root_child0.get_layout_left());
	assert_eq!(0 as f32, root_child0.get_layout_top());
	assert_eq!(60 as f32, root_child0.get_layout_width());
	assert_eq!(40 as f32, root_child0.get_layout_height());
}

#[test]
fn test_absolute_layout_align_items_and_justify_content_center_and_top_position() {
	let mut config = Config::new();

	let mut root = Node::new_with_config(&mut config);
	root.set_justify_content(Justify::Center);
	root.set_align_items(Align::Center);
	root.set_flex_grow(1 as f32);
	root.set_width(StyleUnit::Point((110 as f32).into()));
	root.set_height(StyleUnit::Point((100 as f32).into()));

	let mut root_child0 = Node::new_with_config(&mut config);
	root_child0.set_position_type(PositionType::Absolute);
	root_child0.set_position(Edge::Top, StyleUnit::Point((10 as f32).into()));
	root_child0.set_width(StyleUnit::Point((60 as f32).into()));
	root_child0.set_height(StyleUnit::Point((40 as f32).into()));
	root.insert_child(&mut root_child0, 0);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	assert_eq!(0 as f32, root.get_layout_left());
	assert_eq!(0 as f32, root.get_layout_top());
	assert_eq!(110 as f32, root.get_layout_width());
	assert_eq!(100 as f32, root.get_layout_height());

	assert_eq!(25 as f32, root_child0.get_layout_left());
	assert_eq!(10 as f32, root_child0.get_layout_top());
	assert_eq!(60 as f32, root_child0.get_layout_width());
	assert_eq!(40 as f32, root_child0.get_layout_height());

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	assert_eq!(0 as f32, root.get_layout_left());
	assert_eq!(0 as f32, root.get_layout_top());
	assert_eq!(110 as f32, root.get_layout_width());
	assert_eq!(100 as f32, root.get_layout_height());

	assert_eq!(25 as f32, root_child0.get_layout_left());
	assert_eq!(10 as f32, root_child0.get_layout_top());
	assert_eq!(60 as f32, root_child0.get_layout_width());
	assert_eq!(40 as f32, root_child0.get_layout_height());
}

#[test]
fn test_absolute_layout_align_items_and_justify_content_center_and_bottom_position() {
	let mut config = Config::new();

	let mut root = Node::new_with_config(&mut config);
	root.set_justify_content(Justify::Center);
	root.set_align_items(Align::Center);
	root.set_flex_grow(1 as f32);
	root.set_width(StyleUnit::Point((110 as f32).into()));
	root.set_height(StyleUnit::Point((100 as f32).into()));

	let mut root_child0 = Node::new_with_config(&mut config);
	root_child0.set_position_type(PositionType::Absolute);
	root_child0.set_position(Edge::Bottom, StyleUnit::Point((10 as f32).into()));
	root_child0.set_width(StyleUnit::Point((60 as f32).into()));
	root_child0.set_height(StyleUnit::Point((40 as f32).into()));
	root.insert_child(&mut root_child0, 0);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	assert_eq!(0 as f32, root.get_layout_left());
	assert_eq!(0 as f32, root.get_layout_top());
	assert_eq!(110 as f32, root.get_layout_width());
	assert_eq!(100 as f32, root.get_layout_height());

	assert_eq!(25 as f32, root_child0.get_layout_left());
	assert_eq!(50 as f32, root_child0.get_layout_top());
	assert_eq!(60 as f32, root_child0.get_layout_width());
	assert_eq!(40 as f32, root_child0.get_layout_height());

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	assert_eq!(0 as f32, root.get_layout_left());
	assert_eq!(0 as f32, root.get_layout_top());
	assert_eq!(110 as f32, root.get_layout_width());
	assert_eq!(100 as f32, root.get_layout_height());

	assert_eq!(25 as f32, root_child0.get_layout_left());
	assert_eq!(50 as f32, root_child0.get_layout_top());
	assert_eq!(60 as f32, root_child0.get_layout_width());
	assert_eq!(40 as f32, root_child0.get_layout_height());
}

#[test]
fn test_absolute_layout_align_items_and_justify_content_center_and_left_position() {
	let mut config = Config::new();

	let mut root = Node::new_with_config(&mut config);
	root.set_justify_content(Justify::Center);
	root.set_align_items(Align::Center);
	root.set_flex_grow(1 as f32);
	root.set_width(StyleUnit::Point((110 as f32).into()));
	root.set_height(StyleUnit::Point((100 as f32).into()));

	let mut root_child0 = Node::new_with_config(&mut config);
	root_child0.set_position_type(PositionType::Absolute);
	root_child0.set_position(Edge::Left, StyleUnit::Point((5 as f32).into()));
	root_child0.set_width(StyleUnit::Point((60 as f32).into()));
	root_child0.set_height(StyleUnit::Point((40 as f32).into()));
	root.insert_child(&mut root_child0, 0);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	assert_eq!(0 as f32, root.get_layout_left());
	assert_eq!(0 as f32, root.get_layout_top());
	assert_eq!(110 as f32, root.get_layout_width());
	assert_eq!(100 as f32, root.get_layout_height());

	assert_eq!(5 as f32, root_child0.get_layout_left());
	assert_eq!(30 as f32, root_child0.get_layout_top());
	assert_eq!(60 as f32, root_child0.get_layout_width());
	assert_eq!(40 as f32, root_child0.get_layout_height());

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	assert_eq!(0 as f32, root.get_layout_left());
	assert_eq!(0 as f32, root.get_layout_top());
	assert_eq!(110 as f32, root.get_layout_width());
	assert_eq!(100 as f32, root.get_layout_height());

	assert_eq!(5 as f32, root_child0.get_layout_left());
	assert_eq!(30 as f32, root_child0.get_layout_top());
	assert_eq!(60 as f32, root_child0.get_layout_width());
	assert_eq!(40 as f32, root_child0.get_layout_height());
}

#[test]
fn test_absolute_layout_align_items_and_justify_content_center_and_right_position() {
	let mut config = Config::new();

	let mut root = Node::new_with_config(&mut config);
	root.set_justify_content(Justify::Center);
	root.set_align_items(Align::Center);
	root.set_flex_grow(1 as f32);
	root.set_width(StyleUnit::Point((110 as f32).into()));
	root.set_height(StyleUnit::Point((100 as f32).into()));

	let mut root_child0 = Node::new_with_config(&mut config);
	root_child0.set_position_type(PositionType::Absolute);
	root_child0.set_position(Edge::Right, StyleUnit::Point((5 as f32).into()));
	root_child0.set_width(StyleUnit::Point((60 as f32).into()));
	root_child0.set_height(StyleUnit::Point((40 as f32).into()));
	root.insert_child(&mut root_child0, 0);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	assert_eq!(0 as f32, root.get_layout_left());
	assert_eq!(0 as f32, root.get_layout_top());
	assert_eq!(110 as f32, root.get_layout_width());
	assert_eq!(100 as f32, root.get_layout_height());

	assert_eq!(45 as f32, root_child0.get_layout_left());
	assert_eq!(30 as f32, root_child0.get_layout_top());
	assert_eq!(60 as f32, root_child0.get_layout_width());
	assert_eq!(40 as f32, root_child0.get_layout_height());

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	assert_eq!(0 as f32, root.get_layout_left());
	assert_eq!(0 as f32, root.get_layout_top());
	assert_eq!(110 as f32, root.get_layout_width());
	assert_eq!(100 as f32, root.get_layout_height());

	assert_eq!(45 as f32, root_child0.get_layout_left());
	assert_eq!(30 as f32, root_child0.get_layout_top());
	assert_eq!(60 as f32, root_child0.get_layout_width());
	assert_eq!(40 as f32, root_child0.get_layout_height());
}

#[test]
fn test_position_root_with_rtl_should_position_withoutdirection() {
	let mut config = Config::new();

	let mut root = Node::new_with_config(&mut config);
	root.set_position(Edge::Left, StyleUnit::Point((72 as f32).into()));
	root.set_width(StyleUnit::Point((52 as f32).into()));
	root.set_height(StyleUnit::Point((52 as f32).into()));
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	assert_eq!(72 as f32, root.get_layout_left());
	assert_eq!(0 as f32, root.get_layout_top());
	assert_eq!(52 as f32, root.get_layout_width());
	assert_eq!(52 as f32, root.get_layout_height());

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	assert_eq!(72 as f32, root.get_layout_left());
	assert_eq!(0 as f32, root.get_layout_top());
	assert_eq!(52 as f32, root.get_layout_width());
	assert_eq!(52 as f32, root.get_layout_height());
}

#[test]
fn test_absolute_layout_percentage_bottom_based_on_parent_height() {
	let mut config = Config::new();

	let mut root = Node::new_with_config(&mut config);
	root.set_width(StyleUnit::Point((100 as f32).into()));
	root.set_height(StyleUnit::Point((200 as f32).into()));

	let mut root_child0 = Node::new_with_config(&mut config);
	root_child0.set_position_type(PositionType::Absolute);
	root_child0.set_position(Edge::Top, StyleUnit::Percent((50 as f32).into()));
	root_child0.set_width(StyleUnit::Point((10 as f32).into()));
	root_child0.set_height(StyleUnit::Point((10 as f32).into()));
	root.insert_child(&mut root_child0, 0);

	let mut root_child1 = Node::new_with_config(&mut config);
	root_child1.set_position_type(PositionType::Absolute);
	root_child1.set_position(Edge::Bottom, StyleUnit::Percent((50 as f32).into()));
	root_child1.set_width(StyleUnit::Point((10 as f32).into()));
	root_child1.set_height(StyleUnit::Point((10 as f32).into()));
	root.insert_child(&mut root_child1, 1);

	let mut root_child2 = Node::new_with_config(&mut config);
	root_child2.set_position_type(PositionType::Absolute);
	root_child2.set_position(Edge::Top, StyleUnit::Percent((10 as f32).into()));
	root_child2.set_position(Edge::Bottom, StyleUnit::Percent((10 as f32).into()));
	root_child2.set_width(StyleUnit::Point((10 as f32).into()));
	root.insert_child(&mut root_child2, 2);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	assert_eq!(0 as f32, root.get_layout_left());
	assert_eq!(0 as f32, root.get_layout_top());
	assert_eq!(100 as f32, root.get_layout_width());
	assert_eq!(200 as f32, root.get_layout_height());

	assert_eq!(0 as f32, root_child0.get_layout_left());
	assert_eq!(100 as f32, root_child0.get_layout_top());
	assert_eq!(10 as f32, root_child0.get_layout_width());
	assert_eq!(10 as f32, root_child0.get_layout_height());

	assert_eq!(0 as f32, root_child1.get_layout_left());
	assert_eq!(90 as f32, root_child1.get_layout_top());
	assert_eq!(10 as f32, root_child1.get_layout_width());
	assert_eq!(10 as f32, root_child1.get_layout_height());

	assert_eq!(0 as f32, root_child2.get_layout_left());
	assert_eq!(20 as f32, root_child2.get_layout_top());
	assert_eq!(10 as f32, root_child2.get_layout_width());
	assert_eq!(160 as f32, root_child2.get_layout_height());

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	assert_eq!(0 as f32, root.get_layout_left());
	assert_eq!(0 as f32, root.get_layout_top());
	assert_eq!(100 as f32, root.get_layout_width());
	assert_eq!(200 as f32, root.get_layout_height());

	assert_eq!(90 as f32, root_child0.get_layout_left());
	assert_eq!(100 as f32, root_child0.get_layout_top());
	assert_eq!(10 as f32, root_child0.get_layout_width());
	assert_eq!(10 as f32, root_child0.get_layout_height());

	assert_eq!(90 as f32, root_child1.get_layout_left());
	assert_eq!(90 as f32, root_child1.get_layout_top());
	assert_eq!(10 as f32, root_child1.get_layout_width());
	assert_eq!(10 as f32, root_child1.get_layout_height());

	assert_eq!(90 as f32, root_child2.get_layout_left());
	assert_eq!(20 as f32, root_child2.get_layout_top());
	assert_eq!(10 as f32, root_child2.get_layout_width());
	assert_eq!(160 as f32, root_child2.get_layout_height());
}

#[test]
fn test_absolute_layout_in_wrap_reverse_column_container() {
	let mut config = Config::new();

	let mut root = Node::new_with_config(&mut config);
	root.set_flex_wrap(Wrap::WrapReverse);
	root.set_width(StyleUnit::Point((100 as f32).into()));
	root.set_height(StyleUnit::Point((100 as f32).into()));

	let mut root_child0 = Node::new_with_config(&mut config);
	root_child0.set_position_type(PositionType::Absolute);
	root_child0.set_width(StyleUnit::Point((20 as f32).into()));
	root_child0.set_height(StyleUnit::Point((20 as f32).into()));
	root.insert_child(&mut root_child0, 0);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	assert_eq!(0 as f32, root.get_layout_left());
	assert_eq!(0 as f32, root.get_layout_top());
	assert_eq!(100 as f32, root.get_layout_width());
	assert_eq!(100 as f32, root.get_layout_height());

	assert_eq!(80 as f32, root_child0.get_layout_left());
	assert_eq!(0 as f32, root_child0.get_layout_top());
	assert_eq!(20 as f32, root_child0.get_layout_width());
	assert_eq!(20 as f32, root_child0.get_layout_height());

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	assert_eq!(0 as f32, root.get_layout_left());
	assert_eq!(0 as f32, root.get_layout_top());
	assert_eq!(100 as f32, root.get_layout_width());
	assert_eq!(100 as f32, root.get_layout_height());

	assert_eq!(0 as f32, root_child0.get_layout_left());
	assert_eq!(0 as f32, root_child0.get_layout_top());
	assert_eq!(20 as f32, root_child0.get_layout_width());
	assert_eq!(20 as f32, root_child0.get_layout_height());
}

#[test]
fn test_absolute_layout_in_wrap_reverse_row_container() {
	let mut config = Config::new();

	let mut root = Node::new_with_config(&mut config);
	root.set_flex_direction(FlexDirection::Row);
	root.set_flex_wrap(Wrap::WrapReverse);
	root.set_width(StyleUnit::Point((100 as f32).into()));
	root.set_height(StyleUnit::Point((100 as f32).into()));

	let mut root_child0 = Node::new_with_config(&mut config);
	root_child0.set_position_type(PositionType::Absolute);
	root_child0.set_width(StyleUnit::Point((20 as f32).into()));
	root_child0.set_height(StyleUnit::Point((20 as f32).into()));
	root.insert_child(&mut root_child0, 0);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	assert_eq!(0 as f32, root.get_layout_left());
	assert_eq!(0 as f32, root.get_layout_top());
	assert_eq!(100 as f32, root.get_layout_width());
	assert_eq!(100 as f32, root.get_layout_height());

	assert_eq!(0 as f32, root_child0.get_layout_left());
	assert_eq!(80 as f32, root_child0.get_layout_top());
	assert_eq!(20 as f32, root_child0.get_layout_width());
	assert_eq!(20 as f32, root_child0.get_layout_height());

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	assert_eq!(0 as f32, root.get_layout_left());
	assert_eq!(0 as f32, root.get_layout_top());
	assert_eq!(100 as f32, root.get_layout_width());
	assert_eq!(100 as f32, root.get_layout_height());

	assert_eq!(80 as f32, root_child0.get_layout_left());
	assert_eq!(80 as f32, root_child0.get_layout_top());
	assert_eq!(20 as f32, root_child0.get_layout_width());
	assert_eq!(20 as f32, root_child0.get_layout_height());
}

#[test]
fn test_absolute_layout_in_wrap_reverse_column_container_flex_end() {
	let mut config = Config::new();

	let mut root = Node::new_with_config(&mut config);
	root.set_flex_wrap(Wrap::WrapReverse);
	root.set_width(StyleUnit::Point((100 as f32).into()));
	root.set_height(StyleUnit::Point((100 as f32).into()));

	let mut root_child0 = Node::new_with_config(&mut config);
	root_child0.set_align_self(Align::FlexEnd);
	root_child0.set_position_type(PositionType::Absolute);
	root_child0.set_width(StyleUnit::Point((20 as f32).into()));
	root_child0.set_height(StyleUnit::Point((20 as f32).into()));
	root.insert_child(&mut root_child0, 0);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	assert_eq!(0 as f32, root.get_layout_left());
	assert_eq!(0 as f32, root.get_layout_top());
	assert_eq!(100 as f32, root.get_layout_width());
	assert_eq!(100 as f32, root.get_layout_height());

	assert_eq!(0 as f32, root_child0.get_layout_left());
	assert_eq!(0 as f32, root_child0.get_layout_top());
	assert_eq!(20 as f32, root_child0.get_layout_width());
	assert_eq!(20 as f32, root_child0.get_layout_height());

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	assert_eq!(0 as f32, root.get_layout_left());
	assert_eq!(0 as f32, root.get_layout_top());
	assert_eq!(100 as f32, root.get_layout_width());
	assert_eq!(100 as f32, root.get_layout_height());

	assert_eq!(80 as f32, root_child0.get_layout_left());
	assert_eq!(0 as f32, root_child0.get_layout_top());
	assert_eq!(20 as f32, root_child0.get_layout_width());
	assert_eq!(20 as f32, root_child0.get_layout_height());
}

#[test]
fn test_absolute_layout_in_wrap_reverse_row_container_flex_end() {
	let mut config = Config::new();

	let mut root = Node::new_with_config(&mut config);
	root.set_flex_direction(FlexDirection::Row);
	root.set_flex_wrap(Wrap::WrapReverse);
	root.set_width(StyleUnit::Point((100 as f32).into()));
	root.set_height(StyleUnit::Point((100 as f32).into()));

	let mut root_child0 = Node::new_with_config(&mut config);
	root_child0.set_align_self(Align::FlexEnd);
	root_child0.set_position_type(PositionType::Absolute);
	root_child0.set_width(StyleUnit::Point((20 as f32).into()));
	root_child0.set_height(StyleUnit::Point((20 as f32).into()));
	root.insert_child(&mut root_child0, 0);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	assert_eq!(0 as f32, root.get_layout_left());
	assert_eq!(0 as f32, root.get_layout_top());
	assert_eq!(100 as f32, root.get_layout_width());
	assert_eq!(100 as f32, root.get_layout_height());

	assert_eq!(0 as f32, root_child0.get_layout_left());
	assert_eq!(0 as f32, root_child0.get_layout_top());
	assert_eq!(20 as f32, root_child0.get_layout_width());
	assert_eq!(20 as f32, root_child0.get_layout_height());

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	assert_eq!(0 as f32, root.get_layout_left());
	assert_eq!(0 as f32, root.get_layout_top());
	assert_eq!(100 as f32, root.get_layout_width());
	assert_eq!(100 as f32, root.get_layout_height());

	assert_eq!(80 as f32, root_child0.get_layout_left());
	assert_eq!(0 as f32, root_child0.get_layout_top());
	assert_eq!(20 as f32, root_child0.get_layout_width());
	assert_eq!(20 as f32, root_child0.get_layout_height());
}
