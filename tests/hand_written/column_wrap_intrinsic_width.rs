//! The min-content inline size of a multi-line flex container whose main axis is its block axis is
//! the largest min-content contribution of its items (CSS Flexbox 1 §9.9.2), whatever the source of
//! its block size. Its max-content inline size still wraps against that block size. Chrome agrees.
//! <https://www.w3.org/TR/css-flexbox-1/#intrinsic-cross-sizes>
#[cfg(all(feature = "flexbox", feature = "grid"))]
mod column_wrap_intrinsic_width {
    use taffy::prelude::*;
    use taffy::WritingMode;
    use taffy_test_helpers::{new_test_tree, test_measure_function, TestNodeContext};

    fn column_wrap() -> Style {
        Style {
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            flex_wrap: FlexWrap::Wrap,
            ..Default::default()
        }
    }

    fn fixed(width: f32, height: f32) -> Style {
        Style { size: Size { width: length(width), height: length(height) }, flex_shrink: 0.0, ..Default::default() }
    }

    /// A 50px grid with one `auto` column and an `auto` row holds a column wrap container whose items
    /// are a four-word Ahem text (20px min-content, 80px max-content) and a 40x10 box. The grid re-resolves
    /// its column with the stretched row height, which is too short for the items at min-content width.
    #[test]
    fn stretched_auto_row_does_not_wrap_a_min_content_contribution() {
        let mut tree = new_test_tree();
        let words = "HH\u{200b}HH\u{200b}HH\u{200b}HH".to_string();
        let text = tree
            .new_leaf_with_context(
                Style::default(),
                TestNodeContext::ahem_text(words, taffy_test_helpers::WritingMode::Horizontal),
            )
            .unwrap();
        let block = tree.new_leaf(fixed(40.0, 10.0)).unwrap();
        let flex = tree.new_with_children(column_wrap(), &[text, block]).unwrap();
        let grid = tree
            .new_with_children(
                Style {
                    display: Display::Grid,
                    size: Size { width: length(50.0), height: auto() },
                    ..Default::default()
                },
                &[flex],
            )
            .unwrap();
        tree.compute_layout_with_measure_output(grid, Size::MAX_CONTENT, test_measure_function).unwrap();
        assert_eq!(tree.layout(flex).unwrap().size.width, 50.0);
    }

    /// A 30px grid with one `auto` column and a fixed 50px row holds a column wrap container with two
    /// 20x50 items. At min-content width the items form one column, so the column grows to the grid width.
    #[test]
    fn fixed_row_does_not_wrap_a_min_content_contribution() {
        let mut tree = new_test_tree();
        let first = tree.new_leaf(fixed(20.0, 50.0)).unwrap();
        let second = tree.new_leaf(fixed(20.0, 50.0)).unwrap();
        let flex = tree.new_with_children(column_wrap(), &[first, second]).unwrap();
        let grid = tree
            .new_with_children(
                Style {
                    display: Display::Grid,
                    grid_template_rows: vec![length(50.0)],
                    size: Size { width: length(30.0), height: auto() },
                    ..Default::default()
                },
                &[flex],
            )
            .unwrap();
        tree.compute_layout_with_measure_output(grid, Size::MAX_CONTENT, test_measure_function).unwrap();
        assert_eq!(tree.layout(flex).unwrap().size.width, 30.0);
    }

    /// WPT css-flexbox/intrinsic-size/col-wrap-009: a column wrap child of a `height: 100px` row flex
    /// container is stretched to 100px, and its max-content width wraps two 50x100 items into two columns.
    #[test]
    fn stretched_height_still_wraps_a_max_content_contribution() {
        let mut tree = new_test_tree();
        let first = tree.new_leaf(fixed(50.0, 100.0)).unwrap();
        let second = tree.new_leaf(fixed(50.0, 100.0)).unwrap();
        let flex = tree.new_with_children(column_wrap(), &[first, second]).unwrap();
        let row = tree
            .new_with_children(
                Style {
                    display: Display::Flex,
                    size: Size { width: auto(), height: length(100.0) },
                    ..Default::default()
                },
                &[flex],
            )
            .unwrap();
        tree.compute_layout_with_measure_output(row, Size::MAX_CONTENT, test_measure_function).unwrap();
        assert_eq!(tree.layout(flex).unwrap().size.width, 100.0);
    }

    /// In `vertical-lr` a `row wrap` container's main axis is its inline (physical vertical) axis, so its
    /// physical width is its block size, which still sums its lines: two 20x50 items in a 50px tall
    /// container form two lines, 40px wide.
    #[test]
    fn vertical_row_wrap_block_size_still_sums_its_lines() {
        let mut tree = new_test_tree();
        let first = tree.new_leaf(fixed(20.0, 50.0)).unwrap();
        let second = tree.new_leaf(fixed(20.0, 50.0)).unwrap();
        let flex = tree
            .new_with_children(
                Style {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Row,
                    flex_wrap: FlexWrap::Wrap,
                    size: Size { width: auto(), height: length(50.0) },
                    ..Default::default()
                },
                &[first, second],
            )
            .unwrap();
        for node in [first, second, flex] {
            tree.set_writing_mode(node, WritingMode::VerticalLr).unwrap();
        }
        let grid = tree
            .new_with_children(
                Style {
                    display: Display::Grid,
                    size: Size { width: length(30.0), height: auto() },
                    ..Default::default()
                },
                &[flex],
            )
            .unwrap();
        tree.compute_layout_with_measure_output(grid, Size::MAX_CONTENT, test_measure_function).unwrap();
        assert_eq!(tree.layout(flex).unwrap().size.width, 40.0);
    }

    /// In `vertical-lr` a `column wrap` container's main axis is its block (physical horizontal) axis, so
    /// its min-content inline size is its min-content height. A 30px tall grid with a fixed 50px column and
    /// an `auto` row holds the container with two 50x20 items: at min-content height they form one line,
    /// so the row grows to the grid height.
    #[test]
    fn vertical_column_wrap_min_content_height_is_a_single_line() {
        let mut tree = new_test_tree();
        let first = tree.new_leaf(fixed(50.0, 20.0)).unwrap();
        let second = tree.new_leaf(fixed(50.0, 20.0)).unwrap();
        let flex = tree.new_with_children(column_wrap(), &[first, second]).unwrap();
        for node in [first, second, flex] {
            tree.set_writing_mode(node, WritingMode::VerticalLr).unwrap();
        }
        let grid = tree
            .new_with_children(
                Style {
                    display: Display::Grid,
                    grid_template_columns: vec![length(50.0)],
                    size: Size { width: auto(), height: length(30.0) },
                    ..Default::default()
                },
                &[flex],
            )
            .unwrap();
        tree.compute_layout_with_measure_output(grid, Size::MAX_CONTENT, test_measure_function).unwrap();
        assert_eq!(tree.layout(flex).unwrap().size.height, 30.0);
    }
}
