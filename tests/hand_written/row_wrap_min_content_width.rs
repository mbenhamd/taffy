//! The min-content main size of a multi-line row flex container is the largest min-content
//! contribution among its items (CSS Flexbox 1 §9.9.1). An item's flex basis does not raise that
//! contribution: Chrome sizes a wrapping row holding a `flex: 1 1 320px` item whose content is 20px
//! wide, next to a 100px item, at 100px under a min-content constraint.
//! <https://www.w3.org/TR/css-flexbox-1/#intrinsic-main-sizes>
#[cfg(feature = "flexbox")]
mod row_wrap_min_content_width {
    use taffy::prelude::*;

    fn min_content_width(first_item_shrink: f32, with_second_item: bool) -> f32 {
        let mut tree: TaffyTree<()> = TaffyTree::new();
        let content = tree
            .new_leaf(Style { size: Size { width: length(20.0), height: length(5.0) }, ..Default::default() })
            .unwrap();
        let basis_item = tree
            .new_with_children(
                Style {
                    flex_grow: 1.0,
                    flex_shrink: first_item_shrink,
                    flex_basis: length(320.0),
                    ..Default::default()
                },
                &[content],
            )
            .unwrap();
        let mut children = vec![basis_item];
        if with_second_item {
            children.push(
                tree.new_leaf(Style { size: Size { width: length(100.0), height: length(5.0) }, ..Default::default() })
                    .unwrap(),
            );
        }
        let flex = tree
            .new_with_children(
                Style {
                    display: Display::Flex,
                    flex_wrap: FlexWrap::Wrap,
                    gap: Size { width: length(12.0), height: zero() },
                    ..Default::default()
                },
                &children,
            )
            .unwrap();
        tree.compute_layout(flex, Size { width: AvailableSpace::MinContent, height: AvailableSpace::MaxContent })
            .unwrap();
        tree.layout(flex).unwrap().size.width
    }

    #[test]
    fn a_shrinkable_flex_basis_does_not_widen_the_min_content_size() {
        assert_eq!(min_content_width(1.0, true), 100.0);
        assert_eq!(min_content_width(1.0, false), 20.0);
    }

    #[test]
    fn an_unshrinkable_flex_basis_does_not_widen_the_min_content_size() {
        assert_eq!(min_content_width(0.0, true), 100.0);
    }
}
