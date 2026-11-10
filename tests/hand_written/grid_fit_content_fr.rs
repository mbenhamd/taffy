//! A grid container with an indefinite inline size that is sized into a definite available inline space
//! (fit-content) expands its flexible tracks into that space, not to their max-content size.
//! Expected values are Chrome's.
#[cfg(all(feature = "flexbox", feature = "grid"))]
mod grid_fit_content_fr {
    use taffy::prelude::*;
    use taffy_test_helpers::new_test_tree;

    /// Lays out a 226x300 flex column holding a grid with `columns`, two 137x10 items and an 11.2px column gap.
    /// Returns the widths of the grid and of its two items.
    fn grid_in_flex_column(
        columns: Vec<TrackSizingFunction>,
        wrap: FlexWrap,
        align_items: AlignItems,
    ) -> (f32, f32, f32) {
        let mut tree = new_test_tree();
        tree.disable_rounding();
        let item_style = Style { size: Size { width: length(137.0), height: length(10.0) }, ..Default::default() };
        let first = tree.new_leaf(item_style.clone()).unwrap();
        let second = tree.new_leaf(item_style).unwrap();
        let grid = tree
            .new_with_children(
                Style {
                    display: Display::Grid,
                    grid_template_columns: columns.into_iter().map(Into::into).collect(),
                    gap: Size { width: length(11.2), height: length(0.0) },
                    ..Default::default()
                },
                &[first, second],
            )
            .unwrap();
        let flex = tree
            .new_with_children(
                Style {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    flex_wrap: wrap,
                    align_items,
                    size: Size { width: length(226.0), height: length(300.0) },
                    ..Default::default()
                },
                &[grid],
            )
            .unwrap();
        tree.compute_layout(flex, Size::MAX_CONTENT).unwrap();
        let width = |node| tree.layout(node).unwrap().size.width;
        (width(grid), width(first), width(second))
    }

    fn assert_close(actual: f32, expected: f32) {
        assert!((actual - expected).abs() < 0.05, "expected {expected}, got {actual}");
    }

    fn zero_to_fr() -> TrackSizingFunction {
        flex(1.0)
    }

    fn auto_to_fr(flex_factor: f32) -> TrackSizingFunction {
        minmax(auto(), fr(flex_factor))
    }

    /// A multi-line column container's line is as wide as its widest item's hypothetical (fit-content) width.
    /// Chrome sizes the grid to the 226px space: each `minmax(0, 1fr)` column is (226 - 11.2) / 2 = 107.4 wide.
    #[test]
    fn a_column_wrap_line_holds_a_fit_content_grid_at_the_available_width() {
        let (grid, _, _) = grid_in_flex_column(vec![zero_to_fr(), zero_to_fr()], FlexWrap::Wrap, AlignItems::STRETCH);
        assert_close(grid, 226.0);
    }

    /// A non-stretched flex item's cross size is fit-content: the grid is 226 wide, not its 285.2 max-content width.
    #[test]
    fn a_flex_start_column_item_grid_fits_the_available_width() {
        let (grid, _, _) =
            grid_in_flex_column(vec![zero_to_fr(), zero_to_fr()], FlexWrap::NoWrap, AlignItems::FLEX_START);
        assert_close(grid, 226.0);
    }

    /// `1fr 2fr` columns have an automatic minimum, so the grid's min-content width is 137 + 11.2 + 137 = 285.2.
    /// That exceeds 226, so the fit-content width is 285.2 (not the 422.2 max-content width) and the tracks
    /// keep their 137px base sizes.
    #[test]
    fn a_fit_content_grid_is_no_narrower_than_its_min_content_width() {
        let (grid, _, _) =
            grid_in_flex_column(vec![auto_to_fr(1.0), auto_to_fr(2.0)], FlexWrap::NoWrap, AlignItems::FLEX_START);
        assert_close(grid, 285.2);
    }

    /// Guard: tracks whose fixed minimum already exceeds the space keep it; the grid overflows at 285.2.
    #[test]
    fn fixed_track_minimums_still_overflow_the_available_width() {
        let column: TrackSizingFunction = minmax(length(137.0), fr(1.0));
        let (grid, _, second) =
            grid_in_flex_column(vec![column.clone(), column], FlexWrap::NoWrap, AlignItems::FLEX_START);
        assert_close(grid, 285.2);
        assert_close(second, 137.0);
    }
}
