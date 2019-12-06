#![windows_subsystem = "windows"]

extern crate fui;
extern crate fui_controls;
extern crate fui_controls_media;
extern crate fui_macros;
extern crate typemap;
extern crate winit;

use fui::application::*;
use fui::*;
use fui_controls::*;
use fui_macros::ui;

use std::cell::RefCell;
use std::rc::Rc;
use typemap::TypeMap;

use fui_controls_media::PlayerGl;
//use fui_controls_media::Player;

struct MainViewModel {
    pub player: Rc<RefCell<PlayerGl>>,
    pub texture_id: Property<i32>,
    //pub player: Rc<RefCell<Player>>,
    player_loop_subscription: EventSubscription,
}

impl MainViewModel {
    pub fn new(app: &mut Application) -> Result<Rc<RefCell<Self>>> {
        let player = Rc::new(RefCell::new(PlayerGl::new(
            app.get_drawing_context(),
            app.get_window_manager(),
            app.get_event_loop().unwrap(),
        )?));
        //let player = Rc::new(RefCell::new(Player::new(app.get_drawing_context().clone())));

        let player_copy = Rc::downgrade(&player);
        let player_loop_subscription =
            app.get_event_loop_interation()
                .borrow_mut()
                .subscribe(move |_| {
                    if let Some(player) = player_copy.upgrade() {
                        let res = player.borrow_mut().on_loop_interation();
                        if let Err(err) = res {
                            eprintln!("Player error: {}", err);
                        }
                    }
                });

        Ok(Rc::new(RefCell::new(MainViewModel {
            player,
            texture_id: Property::new(-1),
            player_loop_subscription,
        })))
    }

    pub fn play(&mut self) {
        self.player.borrow_mut().open();
        self.player.borrow_mut().play();
    }

    pub fn stop(&mut self) {
        self.player.borrow_mut().stop();
    }
}

impl RcView for MainViewModel {
    fn to_view(
        view_model: &Rc<RefCell<Self>>,
        _context: ViewContext,
    ) -> Rc<RefCell<dyn ControlObject>> {
        let vm = &mut view_model.borrow_mut();

        let root_control = ui!(
            Grid {
                ScrollViewer {
                    Bitmap { texture_id: &vm.texture_id },
                },
                Horizontal {
                    Button {
                        clicked: Callback::new(view_model, |vm, _| vm.play()),
                        Text { text: "Play" }
                    },
                    Button {
                        clicked: Callback::new(view_model, |vm, _| vm.stop()),
                        Text { text: "Stop" }
                    },
                },
            }
        );

        let root_control_copy = root_control.clone();
        vm.player
            .borrow_mut()
            .texture
            .updated
            .set_vm(&view_model, move |vm, texture_id| {
                vm.texture_id.set(texture_id);
                // TODO: do it on bitmap control instead
                root_control_copy.borrow_mut().set_is_dirty(true);
            });

        root_control
    }
}

fn main() {
    let mut app = Application::new("Example: video").unwrap();

    let main_view_model = MainViewModel::new(&mut app).unwrap();

    {
        let mut window_manager = app.get_window_manager().borrow_mut();
        let window_builder = winit::window::WindowBuilder::new().with_title("GStreamer test");
        window_manager
            .add_window_view_model(
                window_builder,
                app.get_event_loop().unwrap(),
                &main_view_model,
            )
            .unwrap();
    }
    app.run();
}
