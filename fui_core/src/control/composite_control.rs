use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

use crate::control::*;
use crate::events::{ControlEvent, EventContext};
use crate::view::ViewContext;
use crate::{FuiDrawingContext, Point, Rect, Size};

///
/// A helper control for composing complex controls from simpler ones.
///
/// `CompositeControl` allows you to build the visual tree **after** the parent
/// relationship is established, which gives you access to inherited attached values
/// from parent controls.
///
/// This is useful for controls like Menu, where child controls (Menu, MenuItem)
/// need to access shared state from a parent (MenuBar) that was created earlier.
///
/// # Example
///
/// ```rust
/// pub struct Menu { }
///
/// impl Menu {
///     pub fn to_view(self, _style: Option<Box<dyn Style<Self>>>, context: ViewContext) -> Rc<dyn ControlObject> {
///         CompositeControl::new(context, move |ctx: &ControlContext| {
///             let menu_data = ctx.get_inherited_value::<ActiveMenu>();
///             menu_impl(ctx.get_children(), menu_data, true, TypeMap::new())
///         })
///     }
/// }
/// ```
pub struct CompositeControl {
    context: ControlContext,
    visual_tree: RefCell<Option<Rc<dyn ControlObject>>>,
}

impl CompositeControl {
    /// Creates a new CompositeControl.
    ///
    /// The `builder` closure is called when the parent is attached,
    /// giving you access to inherited attached values.
    pub fn new<F>(context: ViewContext, builder: F) -> Rc<Self>
    where
        F: FnOnce(&ControlContext) -> Rc<dyn ControlObject> + 'static,
    {
        let control = Rc::new(CompositeControl {
            context: ControlContext::new(context),
            visual_tree: RefCell::new(None),
        });

        // Set self reference
        let control_weak = Rc::downgrade(&control);
        control.context.set_self(control_weak);

        // Store builder as on_parent_attached callback
        let control_clone = control.clone();
        control.context.set_on_parent_attached(move |ctx| {
            let visual_tree = builder(ctx);

            // Set parent and run setup for the visual tree root itself
            let control_as_dyn: Rc<dyn ControlObject> = control_clone.clone();
            visual_tree.get_context().set_parent(&control_as_dyn);
            visual_tree.get_context().set_services(ctx.get_services());
            visual_tree.setup();

            *control_clone.visual_tree.borrow_mut() = Some(visual_tree);
        });

        control
    }

    pub fn get_context(&self) -> &ControlContext {
        &self.context
    }

    fn with_visual_tree<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&Rc<dyn ControlObject>) -> R,
    {
        let visual_tree = self.visual_tree.borrow();
        if let Some(ref vt) = *visual_tree {
            f(vt)
        } else {
            panic!("CompositeControl: visual tree not yet initialized");
        }
    }
}

impl ControlObject for CompositeControl {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn get_context(&self) -> &ControlContext {
        &self.context
    }
}

impl ControlBehavior for CompositeControl {
    fn parent_attached(&self) {
        let services = self.context.get_services();
        self.with_visual_tree(|vt| {
            vt.get_context().set_services(services);
            vt.get_context().attach_tree();
        });
    }

    fn parent_detached(&self) {
        self.with_visual_tree(|vt| vt.get_context().detach_tree());
    }

    fn setup(&self) {
        self.with_visual_tree(|vt| vt.setup());
    }

    fn handle_event(
        &self,
        drawing_context: &mut FuiDrawingContext,
        event_context: &mut dyn EventContext,
        event: ControlEvent,
    ) {
        self.with_visual_tree(|vt| {
            vt.handle_event(drawing_context, event_context, event);
        });
    }

    fn measure(&self, drawing_context: &mut FuiDrawingContext, size: Size) {
        self.with_visual_tree(|vt| {
            vt.measure(drawing_context, size);
        });
    }

    fn set_rect(&self, drawing_context: &mut FuiDrawingContext, rect: Rect) {
        self.context.set_rect(rect);
        self.with_visual_tree(|vt| {
            vt.set_rect(drawing_context, rect);
        });
    }

    fn get_rect(&self) -> Rect {
        self.with_visual_tree(|vt| vt.get_rect())
    }

    fn hit_test(&self, point: Point) -> Option<Rc<dyn ControlObject>> {
        self.with_visual_tree(|vt| vt.hit_test(point))
    }

    fn draw(&self, drawing_context: &mut FuiDrawingContext) {
        self.with_visual_tree(|vt| vt.draw(drawing_context));
    }
}
