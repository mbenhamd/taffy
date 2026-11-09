#[cfg(feature = "grid")]
mod grid_min_size_contribution {
    use taffy::prelude::*;
    use taffy_test_helpers::new_test_tree;

    /// An outer grid's auto track holds an inner grid with one `1fr` column, whose item
    /// declares `min-width`. Returns the inner grid's width.
    fn inner_grid_width(item_min_width: LengthPercentageAuto) -> f32 {
        let mut tree = new_test_tree();
        let content = tree
            .new_leaf(Style { size: Size { width: length(523.0), height: length(10.0) }, ..Default::default() })
            .unwrap();
        let item = tree
            .new_with_children(
                Style {
                    min_size: Size { width: item_min_width, height: LengthPercentageAuto::auto() },
                    ..Default::default()
                },
                &[content],
            )
            .unwrap();
        let inner = tree
            .new_with_children(
                Style { display: Display::Grid, grid_template_columns: vec![fr(1.0)], ..Default::default() },
                &[item],
            )
            .unwrap();
        let outer = tree
            .new_with_children(
                Style {
                    display: Display::Grid,
                    size: Size { width: length(369.0), height: auto() },
                    ..Default::default()
                },
                &[inner],
            )
            .unwrap();
        tree.compute_layout(outer, Size::MAX_CONTENT).unwrap();
        tree.layout(inner).unwrap().size.width
    }

    #[test]
    fn explicit_min_width_bounds_the_min_content_contribution_through_a_flexible_track() {
        assert_eq!(inner_grid_width(LengthPercentageAuto::length(0.0)), 369.0);
    }

    #[test]
    fn automatic_min_width_keeps_the_content_based_minimum() {
        assert_eq!(inner_grid_width(LengthPercentageAuto::auto()), 523.0);
    }
}
