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

    /// The measure pass sizes the area from the 70.1px row, the 11.2px gap with its `space-evenly` adjustment and
    /// the second 70.1px row, summed element by element. The final pass must sum the same elements in the same
    /// order: summing the base sizes and the distributed space separately is 1 ulp short in f32 (167.59999 vs
    /// 167.60001), which wraps the second 83.8px child of the stretched column wrap container into a second
    /// column (two children of 83.8 add up to 167.60001).
    #[test]
    fn a_spanning_column_wrap_item_keeps_its_measured_height_under_space_evenly() {
        let mut tree = new_test_tree();
        tree.disable_rounding();
        let children: Vec<NodeId> = (0..2)
            .map(|_| {
                tree.new_leaf(Style {
                    size: Size { width: length(100.0), height: length(83.8) },
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
                    grid_row: Line { start: line(1), end: line(3) },
                    ..Default::default()
                },
                &children,
            )
            .unwrap();
        let grid = tree
            .new_with_children(
                Style {
                    display: Display::Grid,
                    grid_template_rows: vec![length(70.1), length(70.1)],
                    gap: Size { width: length(0.0), height: length(11.2) },
                    align_content: AlignContent::SPACE_EVENLY,
                    size: Size { width: length(300.0), height: length(200.0) },
                    ..Default::default()
                },
                &[flex],
            )
            .unwrap();
        tree.compute_layout(grid, Size::MAX_CONTENT).unwrap();
        let xs: Vec<f32> = children.iter().map(|&child| tree.layout(child).unwrap().location.x).collect();
        assert_eq!(xs, vec![0.0, 0.0]);
    }

    /// The column analogue under `space-around`, in both directions: the area spans 40px, the 10.3px gap with its
    /// share of the distributed space, and 40.1px. In RTL that share is the space placed before the logically first
    /// column, which is physically after the gap. Summed like the measure pass, the area holds the 10.1px and
    /// 135.1px children of the row wrap container on one line.
    #[test]
    fn a_spanning_row_wrap_item_keeps_its_measured_width_under_space_around_in_both_directions() {
        for direction in [Direction::Ltr, Direction::Rtl] {
            let mut tree = new_test_tree();
            tree.disable_rounding();
            let children: Vec<NodeId> = [10.1, 135.1]
                .into_iter()
                .map(|width| {
                    tree.new_leaf(Style {
                        size: Size { width: length(width), height: length(20.0) },
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
                        flex_direction: FlexDirection::Row,
                        flex_wrap: FlexWrap::Wrap,
                        grid_column: Line { start: line(1), end: line(3) },
                        ..Default::default()
                    },
                    &children,
                )
                .unwrap();
            let grid = tree
                .new_with_children(
                    Style {
                        display: Display::Grid,
                        direction,
                        grid_template_columns: vec![length(40.0), length(40.1)],
                        gap: Size { width: length(10.3), height: length(0.0) },
                        justify_content: JustifyContent::SPACE_AROUND,
                        size: Size { width: length(200.0), height: auto() },
                        ..Default::default()
                    },
                    &[flex],
                )
                .unwrap();
            tree.compute_layout(grid, Size::MAX_CONTENT).unwrap();
            let ys: Vec<f32> = children.iter().map(|&child| tree.layout(child).unwrap().location.y).collect();
            assert_eq!(ys, vec![0.0, 0.0], "{direction:?}");
        }
    }
}
