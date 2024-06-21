use crate::elements::container::Container;
use crate::elements::element::Element;
use crate::elements::empty::Empty;
use crate::elements::text::Text;
use crate::elements::trees::diff_tree;
use crate::widget_id::reset_unique_widget_id;

#[test]
fn diff_assigns_stable_id_when_child_is_removed() {
    let mut old_a = Container::new();
    let old_b = Text::new("b");
    let old_c = Text::new("c");

    old_a = old_a.add_child(old_b.into());
    old_a = old_a.add_child(old_c.into());
    
    let old_a: Box<dyn Element> = old_a.into();
    old_a.print_tree();
    
    // Reset the new widget id back to zero, act like this is a new render.
    reset_unique_widget_id();
    
    let mut new_a = Container::new();
    let new_b: Box<dyn Element> = Empty::new().into();
    let new_c = Text::new("c").into();
    new_a = new_a.add_child(new_b);
    new_a = new_a.add_child(new_c);
    let new_a: Box<dyn Element> = new_a.into();

    new_a.print_tree();
    
    //let new_tree = diff_tree(Some(&mut old_a), Some(&mut new_a));

    assert_eq!(old_a.children()[1].id(), new_a.children()[1].id(), "test that b has the same id when removed");
}
