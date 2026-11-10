//! A grid item's area is sized from the tracks it spans, not from the difference of its edge offsets.
#[cfg(all(feature = "flexbox", feature = "grid"))]
mod grid_area_size {
    use taffy::prelude::*;
    use taffy::style::Direction;
    use taffy_test_helpers::new_test_tree;

    /// The second row starts at 139.594 + 12.8 = 152.394, and its height is the 205.8px max-content height
    /// of the column wrap container it holds. In f32 the difference of the row's edge offsets is 2 ulp short
    /// of that height. Stretched into the row, the container still fits its items on one line.
    #[test]
    fn a_stretched_column_wrap_container_keeps_the_row_height_it_was_measured_with() {
        let mut tree = new_test_tree();
        let items: Vec<NodeId> = [69.2, 70.2, 44.0]
            .into_iter()
            .map(|height| {
                tree.new_leaf(Style {
                    size: Size { width: length(100.0), height: length(height) },
                    flex_shrink: 0.0,
                    ..Default::default()
                })
                .unwrap()
            })
            .collect();
        let flex = tree
            .new_with_children(
                Style {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    flex_wrap: FlexWrap::Wrap,
                    gap: Size { width: length(0.0), height: length(11.2) },
                    grid_row: Line { start: line(2), end: line(3) },
                    ..Default::default()
                },
                &items,
            )
            .unwrap();
        let grid = tree
            .new_with_children(
                Style {
                    display: Display::Grid,
                    grid_template_rows: vec![length(139.594), auto()],
                    gap: Size { width: length(0.0), height: length(12.8) },
                    size: Size { width: length(300.0), height: auto() },
                    ..Default::default()
                },
                &[flex],
            )
            .unwrap();
        tree.compute_layout(grid, Size::MAX_CONTENT).unwrap();
        let xs: Vec<f32> = items.iter().map(|&item| tree.layout(item).unwrap().location.x).collect();
        assert_eq!(xs, vec![0.0, 0.0, 0.0]);
    }

    /// A 200x300 RTL grid with two 40px columns, two 50px rows and 10px gaps distributes its free space with
    /// `space-around`. An item spanning both tracks in each axis covers the space distributed between them:
    /// 40 + 10 + 55 + 40 = 145 wide and 50 + 10 + 95 + 50 = 205 tall, as in Chrome.
    #[test]
    fn a_spanning_item_includes_the_space_distributed_between_its_tracks() {
        let mut tree = new_test_tree();
        tree.disable_rounding();
        let item = tree
            .new_leaf(Style {
                grid_row: Line { start: line(1), end: line(3) },
                grid_column: Line { start: line(1), end: line(3) },
                ..Default::default()
            })
            .unwrap();
        let grid = tree
            .new_with_children(
                Style {
                    display: Display::Grid,
                    direction: Direction::Rtl,
                    grid_template_rows: vec![length(50.0), length(50.0)],
                    grid_template_columns: vec![length(40.0), length(40.0)],
                    gap: Size { width: length(10.0), height: length(10.0) },
                    align_content: AlignContent::SPACE_AROUND,
                    justify_content: JustifyContent::SPACE_AROUND,
                    size: Size { width: length(200.0), height: length(300.0) },
                    ..Default::default()
                },
                &[item],
            )
            .unwrap();
        tree.compute_layout(grid, Size::MAX_CONTENT).unwrap();
        let layout = tree.layout(item).unwrap();
        assert_eq!((layout.location.x, layout.location.y), (27.5, 47.5));
        assert_eq!((layout.size.width, layout.size.height), (145.0, 205.0));
    }
}
