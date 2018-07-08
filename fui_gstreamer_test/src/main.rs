#![windows_subsystem = "windows"]

extern crate fui;

mod player;
mod pipeline_factory;

use fui::application::*;
use fui::controls::*;
use fui::layout::*;
use fui::*;

use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{ Arc, Mutex };

use fui::Property;
use player::Player;

struct MainViewModel {
    pub player: Rc<RefCell<Player>>,
    player_loop_subscription: EventSubscription,
}

impl MainViewModel {
    pub fn new(app: &mut Application) -> Self {
        let player = Rc::new(RefCell::new(Player::new(app.get_drawing_context())));

        let player_copy = Rc::downgrade(&player);
        let player_loop_subscription = app.get_events_loop_interation().subscribe(move |_| {
            if let Some(player) = player_copy.upgrade() {
                player.borrow_mut().on_loop_interation();
            }
        });

        MainViewModel {
            player,
            player_loop_subscription,
        }
    }

    pub fn play(&mut self) {
        self.player.borrow_mut().open();
        self.player.borrow_mut().play();
    }

    pub fn stop(&mut self) {
        self.player.borrow_mut().stop();
    }
}

impl View for MainViewModel {
    fn create_view(view_model: &Rc<RefCell<MainViewModel>>) -> ViewData {
        // controls
        let btn_play = Button::control(Text::control("Play"));
        let btn_stop = Button::control(Text::control("Stop"));
        let bitmap = Bitmap::control(-1);

        // events
        btn_play.borrow_mut().data.events.clicked.set_vm(view_model, |vm, _| { vm.play(); });
        btn_stop.borrow_mut().data.events.clicked.set_vm(view_model, |vm, _| { vm.stop(); });
        {
            let vm: &mut MainViewModel = &mut view_model.borrow_mut();
            let player = &mut vm.player.borrow_mut();
            player.texture.updated.set_vm(&bitmap, |bitmap, texture_id| {
                bitmap.data.texture_id.set(texture_id);
                bitmap.set_is_dirty(true);
            });
        }

        // bindings
        let vm: &mut MainViewModel = &mut view_model.borrow_mut();
        let bindings = vec![
        ];

        // layout
        let root_control = Horizontal::control(vec![
            btn_play, btn_stop, bitmap,
        ]);

        ViewData {
            root_control: root_control,
            bindings: bindings,
        }
    }
}

fn main() {
    let mut app = Application::new("Marek Ogarek");

    let main_view_model = Rc::new(RefCell::new(MainViewModel::new(&mut app)));
    app.set_root_view_model(&main_view_model);
 
    app.run();  
}
