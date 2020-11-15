/**
 * Copyright (c) 2014-present, Facebook, Inc.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

// @Generated by gentest/gentest.rb from gentest/fixtures/YGDisplayTest.html

extern crate ordered_float;
extern crate djed_yoga;

use djed_yoga::*;

#[test]
fn test_display_none() {
	let mut config = Config::new();

	let mut root = Node::new_with_config(&mut config);
	root.set_flex_direction(FlexDirection::Row);
	root.set_width(StyleUnit::Point((100 as f32).into()));
	root.set_height(StyleUnit::Point((100 as f32).into()));

	let mut root_child0 = Node::new_with_config(&mut config);
	root_child0.set_flex_grow(1 as f32);
	root_child0.set_min_width(StyleUnit::Auto);
	root_child0.set_min_height(StyleUnit::Auto);
	root.insert_child(&mut root_child0, 0);

	let mut root_child1 = Node::new_with_config(&mut config);
	root_child1.set_flex_grow(1 as f32);
	root_child1.set_display(Display::None);
	root.insert_child(&mut root_child1, 1);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	assert_eq!(0 as f32, root.get_layout_left());
	assert_eq!(0 as f32, root.get_layout_top());
	assert_eq!(100 as f32, root.get_layout_width());
	assert_eq!(100 as f32, root.get_layout_height());

	assert_eq!(0 as f32, root_child0.get_layout_left());
	assert_eq!(0 as f32, root_child0.get_layout_top());
	assert_eq!(100 as f32, root_child0.get_layout_width());
	assert_eq!(100 as f32, root_child0.get_layout_height());

	assert_eq!(0 as f32, root_child1.get_layout_left());
	assert_eq!(0 as f32, root_child1.get_layout_top());
	assert_eq!(0 as f32, root_child1.get_layout_width());
	assert_eq!(0 as f32, root_child1.get_layout_height());

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	assert_eq!(0 as f32, root.get_layout_left());
	assert_eq!(0 as f32, root.get_layout_top());
	assert_eq!(100 as f32, root.get_layout_width());
	assert_eq!(100 as f32, root.get_layout_height());

	assert_eq!(0 as f32, root_child0.get_layout_left());
	assert_eq!(0 as f32, root_child0.get_layout_top());
	assert_eq!(100 as f32, root_child0.get_layout_width());
	assert_eq!(100 as f32, root_child0.get_layout_height());

	assert_eq!(0 as f32, root_child1.get_layout_left());
	assert_eq!(0 as f32, root_child1.get_layout_top());
	assert_eq!(0 as f32, root_child1.get_layout_width());
	assert_eq!(0 as f32, root_child1.get_layout_height());
}

#[test]
fn test_display_none_fixed_size() {
	let mut config = Config::new();

	let mut root = Node::new_with_config(&mut config);
	root.set_flex_direction(FlexDirection::Row);
	root.set_width(StyleUnit::Point((100 as f32).into()));
	root.set_height(StyleUnit::Point((100 as f32).into()));

	let mut root_child0 = Node::new_with_config(&mut config);
	root_child0.set_flex_grow(1 as f32);
	root_child0.set_min_width(StyleUnit::Auto);
	root_child0.set_min_height(StyleUnit::Auto);
	root.insert_child(&mut root_child0, 0);

	let mut root_child1 = Node::new_with_config(&mut config);
	root_child1.set_width(StyleUnit::Point((20 as f32).into()));
	root_child1.set_height(StyleUnit::Point((20 as f32).into()));
	root_child1.set_display(Display::None);
	root.insert_child(&mut root_child1, 1);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	assert_eq!(0 as f32, root.get_layout_left());
	assert_eq!(0 as f32, root.get_layout_top());
	assert_eq!(100 as f32, root.get_layout_width());
	assert_eq!(100 as f32, root.get_layout_height());

	assert_eq!(0 as f32, root_child0.get_layout_left());
	assert_eq!(0 as f32, root_child0.get_layout_top());
	assert_eq!(100 as f32, root_child0.get_layout_width());
	assert_eq!(100 as f32, root_child0.get_layout_height());

	assert_eq!(0 as f32, root_child1.get_layout_left());
	assert_eq!(0 as f32, root_child1.get_layout_top());
	assert_eq!(0 as f32, root_child1.get_layout_width());
	assert_eq!(0 as f32, root_child1.get_layout_height());

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	assert_eq!(0 as f32, root.get_layout_left());
	assert_eq!(0 as f32, root.get_layout_top());
	assert_eq!(100 as f32, root.get_layout_width());
	assert_eq!(100 as f32, root.get_layout_height());

	assert_eq!(0 as f32, root_child0.get_layout_left());
	assert_eq!(0 as f32, root_child0.get_layout_top());
	assert_eq!(100 as f32, root_child0.get_layout_width());
	assert_eq!(100 as f32, root_child0.get_layout_height());

	assert_eq!(0 as f32, root_child1.get_layout_left());
	assert_eq!(0 as f32, root_child1.get_layout_top());
	assert_eq!(0 as f32, root_child1.get_layout_width());
	assert_eq!(0 as f32, root_child1.get_layout_height());
}

#[test]
fn test_display_none_with_margin() {
	let mut config = Config::new();

	let mut root = Node::new_with_config(&mut config);
	root.set_flex_direction(FlexDirection::Row);
	root.set_width(StyleUnit::Point((100 as f32).into()));
	root.set_height(StyleUnit::Point((100 as f32).into()));

	let mut root_child0 = Node::new_with_config(&mut config);
	root_child0.set_margin(Edge::Left, StyleUnit::Point((10 as f32).into()));
	root_child0.set_margin(Edge::Top, StyleUnit::Point((10 as f32).into()));
	root_child0.set_margin(Edge::Right, StyleUnit::Point((10 as f32).into()));
	root_child0.set_margin(Edge::Bottom, StyleUnit::Point((10 as f32).into()));
	root_child0.set_width(StyleUnit::Point((20 as f32).into()));
	root_child0.set_height(StyleUnit::Point((20 as f32).into()));
	root_child0.set_display(Display::None);
	root.insert_child(&mut root_child0, 0);

	let mut root_child1 = Node::new_with_config(&mut config);
	root_child1.set_flex_grow(1 as f32);
	root_child1.set_min_width(StyleUnit::Auto);
	root_child1.set_min_height(StyleUnit::Auto);
	root.insert_child(&mut root_child1, 1);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	assert_eq!(0 as f32, root.get_layout_left());
	assert_eq!(0 as f32, root.get_layout_top());
	assert_eq!(100 as f32, root.get_layout_width());
	assert_eq!(100 as f32, root.get_layout_height());

	assert_eq!(0 as f32, root_child0.get_layout_left());
	assert_eq!(0 as f32, root_child0.get_layout_top());
	assert_eq!(0 as f32, root_child0.get_layout_width());
	assert_eq!(0 as f32, root_child0.get_layout_height());

	assert_eq!(0 as f32, root_child1.get_layout_left());
	assert_eq!(0 as f32, root_child1.get_layout_top());
	assert_eq!(100 as f32, root_child1.get_layout_width());
	assert_eq!(100 as f32, root_child1.get_layout_height());

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	assert_eq!(0 as f32, root.get_layout_left());
	assert_eq!(0 as f32, root.get_layout_top());
	assert_eq!(100 as f32, root.get_layout_width());
	assert_eq!(100 as f32, root.get_layout_height());

	assert_eq!(0 as f32, root_child0.get_layout_left());
	assert_eq!(0 as f32, root_child0.get_layout_top());
	assert_eq!(0 as f32, root_child0.get_layout_width());
	assert_eq!(0 as f32, root_child0.get_layout_height());

	assert_eq!(0 as f32, root_child1.get_layout_left());
	assert_eq!(0 as f32, root_child1.get_layout_top());
	assert_eq!(100 as f32, root_child1.get_layout_width());
	assert_eq!(100 as f32, root_child1.get_layout_height());
}

#[test]
fn test_display_none_with_child() {
	let mut config = Config::new();

	let mut root = Node::new_with_config(&mut config);
	root.set_flex_direction(FlexDirection::Row);
	root.set_width(StyleUnit::Point((100 as f32).into()));
	root.set_height(StyleUnit::Point((100 as f32).into()));

	let mut root_child0 = Node::new_with_config(&mut config);
	root_child0.set_flex_grow(1 as f32);
	root_child0.set_flex_shrink(1 as f32 as f32);
	root_child0.set_flex_basis(StyleUnit::Percent((0 as f32).into()));
	root_child0.set_min_width(StyleUnit::Auto);
	root_child0.set_min_height(StyleUnit::Auto);
	root.insert_child(&mut root_child0, 0);

	let mut root_child1 = Node::new_with_config(&mut config);
	root_child1.set_flex_grow(1 as f32);
	root_child1.set_flex_shrink(1 as f32 as f32);
	root_child1.set_flex_basis(StyleUnit::Percent((0 as f32).into()));
	root_child1.set_display(Display::None);
	root.insert_child(&mut root_child1, 1);

	let mut root_child1_child0 = Node::new_with_config(&mut config);
	root_child1_child0.set_flex_grow(1 as f32);
	root_child1_child0.set_flex_shrink(1 as f32 as f32);
	root_child1_child0.set_flex_basis(StyleUnit::Percent((0 as f32).into()));
	root_child1_child0.set_width(StyleUnit::Point((20 as f32).into()));
	root_child1.insert_child(&mut root_child1_child0, 0);

	let mut root_child2 = Node::new_with_config(&mut config);
	root_child2.set_flex_grow(1 as f32);
	root_child2.set_flex_shrink(1 as f32 as f32);
	root_child2.set_flex_basis(StyleUnit::Percent((0 as f32).into()));
	root_child2.set_min_width(StyleUnit::Auto);
	root_child2.set_min_height(StyleUnit::Auto);
	root.insert_child(&mut root_child2, 2);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	assert_eq!(0 as f32, root.get_layout_left());
	assert_eq!(0 as f32, root.get_layout_top());
	assert_eq!(100 as f32, root.get_layout_width());
	assert_eq!(100 as f32, root.get_layout_height());

	assert_eq!(0 as f32, root_child0.get_layout_left());
	assert_eq!(0 as f32, root_child0.get_layout_top());
	assert_eq!(50 as f32, root_child0.get_layout_width());
	assert_eq!(100 as f32, root_child0.get_layout_height());

	assert_eq!(0 as f32, root_child1.get_layout_left());
	assert_eq!(0 as f32, root_child1.get_layout_top());
	assert_eq!(0 as f32, root_child1.get_layout_width());
	assert_eq!(0 as f32, root_child1.get_layout_height());

	assert_eq!(0 as f32, root_child1_child0.get_layout_left());
	assert_eq!(0 as f32, root_child1_child0.get_layout_top());
	assert_eq!(0 as f32, root_child1_child0.get_layout_width());
	assert_eq!(0 as f32, root_child1_child0.get_layout_height());

	assert_eq!(50 as f32, root_child2.get_layout_left());
	assert_eq!(0 as f32, root_child2.get_layout_top());
	assert_eq!(50 as f32, root_child2.get_layout_width());
	assert_eq!(100 as f32, root_child2.get_layout_height());

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	assert_eq!(0 as f32, root.get_layout_left());
	assert_eq!(0 as f32, root.get_layout_top());
	assert_eq!(100 as f32, root.get_layout_width());
	assert_eq!(100 as f32, root.get_layout_height());

	assert_eq!(50 as f32, root_child0.get_layout_left());
	assert_eq!(0 as f32, root_child0.get_layout_top());
	assert_eq!(50 as f32, root_child0.get_layout_width());
	assert_eq!(100 as f32, root_child0.get_layout_height());

	assert_eq!(0 as f32, root_child1.get_layout_left());
	assert_eq!(0 as f32, root_child1.get_layout_top());
	assert_eq!(0 as f32, root_child1.get_layout_width());
	assert_eq!(0 as f32, root_child1.get_layout_height());

	assert_eq!(0 as f32, root_child1_child0.get_layout_left());
	assert_eq!(0 as f32, root_child1_child0.get_layout_top());
	assert_eq!(0 as f32, root_child1_child0.get_layout_width());
	assert_eq!(0 as f32, root_child1_child0.get_layout_height());

	assert_eq!(0 as f32, root_child2.get_layout_left());
	assert_eq!(0 as f32, root_child2.get_layout_top());
	assert_eq!(50 as f32, root_child2.get_layout_width());
	assert_eq!(100 as f32, root_child2.get_layout_height());
}

#[test]
fn test_display_none_with_position() {
	let mut config = Config::new();

	let mut root = Node::new_with_config(&mut config);
	root.set_flex_direction(FlexDirection::Row);
	root.set_width(StyleUnit::Point((100 as f32).into()));
	root.set_height(StyleUnit::Point((100 as f32).into()));

	let mut root_child0 = Node::new_with_config(&mut config);
	root_child0.set_flex_grow(1 as f32);
	root_child0.set_min_width(StyleUnit::Auto);
	root_child0.set_min_height(StyleUnit::Auto);
	root.insert_child(&mut root_child0, 0);

	let mut root_child1 = Node::new_with_config(&mut config);
	root_child1.set_flex_grow(1 as f32);
	root_child1.set_position(Edge::Top, StyleUnit::Point((10 as f32).into()));
	root_child1.set_display(Display::None);
	root.insert_child(&mut root_child1, 1);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	assert_eq!(0 as f32, root.get_layout_left());
	assert_eq!(0 as f32, root.get_layout_top());
	assert_eq!(100 as f32, root.get_layout_width());
	assert_eq!(100 as f32, root.get_layout_height());

	assert_eq!(0 as f32, root_child0.get_layout_left());
	assert_eq!(0 as f32, root_child0.get_layout_top());
	assert_eq!(100 as f32, root_child0.get_layout_width());
	assert_eq!(100 as f32, root_child0.get_layout_height());

	assert_eq!(0 as f32, root_child1.get_layout_left());
	assert_eq!(0 as f32, root_child1.get_layout_top());
	assert_eq!(0 as f32, root_child1.get_layout_width());
	assert_eq!(0 as f32, root_child1.get_layout_height());

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	assert_eq!(0 as f32, root.get_layout_left());
	assert_eq!(0 as f32, root.get_layout_top());
	assert_eq!(100 as f32, root.get_layout_width());
	assert_eq!(100 as f32, root.get_layout_height());

	assert_eq!(0 as f32, root_child0.get_layout_left());
	assert_eq!(0 as f32, root_child0.get_layout_top());
	assert_eq!(100 as f32, root_child0.get_layout_width());
	assert_eq!(100 as f32, root_child0.get_layout_height());

	assert_eq!(0 as f32, root_child1.get_layout_left());
	assert_eq!(0 as f32, root_child1.get_layout_top());
	assert_eq!(0 as f32, root_child1.get_layout_width());
	assert_eq!(0 as f32, root_child1.get_layout_height());
}
