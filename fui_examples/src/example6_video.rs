#![windows_subsystem = "windows"]

use anyhow::Result;
use fui_app::*;
use fui_controls::*;
use fui_core::*;
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
    /*pub fn new(app: &mut Application) -> Result<Rc<RefCell<Self>>> {
        let player = Rc::new(RefCell::new(PlayerGl::new(
            app.get_window_manager().get_drawing_context(),
            app.get_window_manager(),
            app.get_event_loop().unwrap(),
        )?));
        //let player = Rc::new(RefCell::new(Player::new(app.get_resources().clone())));

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
    }*/

    pub fn play(&mut self) {
        self.player.borrow_mut().open();
        self.player.borrow_mut().play();
    }

    pub fn stop(&mut self) {
        self.player.borrow_mut().stop();
    }
}

impl ViewModel for MainViewModel {
    fn create_view(view_model: &Rc<RefCell<Self>>) -> Rc<RefCell<dyn ControlObject>> {
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
                root_control_copy
                    .borrow_mut()
                    .get_context_mut()
                    .set_is_dirty(true);
            });

        root_control
    }
}

fn main() -> Result<()> {
    let mut app = Application::new("Example: video").unwrap();

    let mut window = fui_system::Window::new(None).unwrap();
    window.set_title("GStreamer test");
    window.resize(800, 600);

    //app.add_window(window, MainViewModel::new(&mut app)?)?;

    app.run();

    Ok(())
}
