use std::collections::HashMap;

use crate::entity::fps::FPS;

pub trait State<E>
where
    E: Eq + std::hash::Hash + std::fmt::Debug + Clone,
{
    fn load(self: &mut Self);
    fn key_handler(self: &mut Self, stmc: &mut StateMachineController<E>);
    fn draw(self: &mut Self, dt: f32, stmc: &mut StateMachineController<E>);
}

pub struct StateMachineController<E>
where
    E: Clone,
{
    current: Option<E>,
    exit: bool,
}

pub struct StateMachine<E>
where
    E: Eq + std::hash::Hash + std::fmt::Debug + Clone,
{
    states: HashMap<E, Box<dyn State<E>>>,
    current: Option<E>,
    exit: bool,
    fps_label: FPS,
}

impl<E> StateMachineController<E>
where
    E: Clone,
{
    pub fn new(current: Option<E>) -> Self {
        Self {
            current,
            exit: false,
        }
    }

    pub fn set_state(&mut self, state_id: E) {
        self.current = Some(state_id);
    }

    pub fn state(&self) -> E {
        self.current.clone().unwrap()
    }

    pub fn send_exit(&mut self) {
        self.exit = true;
    }

    pub fn exit_signal(&self) -> bool {
        self.exit
    }
}

impl<E> StateMachine<E>
where
    E: Eq + std::hash::Hash + std::fmt::Debug + Clone,
{
    pub fn new(show_fps: bool) -> Self {
        let mut fps_label = FPS::default();
        fps_label.set_show(show_fps);
        Self {
            states: HashMap::new(),
            current: None,
            exit: false,
            fps_label,
        }
    }

    pub fn state(&self) -> Option<E> {
        self.current.clone()
    }

    pub fn wait_exit_signal(&self) -> bool {
        !self.exit
    }

    pub fn set_state(&mut self, state_id: E) {
        if !self.states.contains_key(&state_id) {
            panic!("State: {:?} not found.", state_id);
        }
        self.current = Some(state_id.clone());
        self.states.get_mut(&state_id).unwrap().load();
    }

    pub fn add_state<T>(&mut self, state_id: E, state: T)
    where
        T: 'static + State<E>,
    {
        self.states.insert(state_id, Box::new(state));
    }

    pub fn handle_keys(&mut self) {
        let mut stmc = StateMachineController::new(self.current.clone());
        self.states
            .get_mut(&self.current.as_ref().unwrap())
            .unwrap()
            .key_handler(&mut stmc);
        self.current = Some(stmc.state());
        self.exit = stmc.exit_signal();
    }

    pub fn draw(&mut self, dt: f32) {
        if !self.states.contains_key(&self.current.as_ref().unwrap()) {
            panic!("State: {:?} not found.", self.current);
        }

        let mut stmc = StateMachineController::new(self.current.clone());
        self.states
            .get_mut(&self.current.as_ref().unwrap())
            .unwrap()
            .draw(dt, &mut stmc);
        self.fps_label.draw();

        if self.current != Some(stmc.state()) {
            self.current = Some(stmc.state());
            self.states
                .get_mut(&self.current.as_ref().unwrap())
                .unwrap()
                .load();
        }
        self.exit = stmc.exit_signal();
    }
}
