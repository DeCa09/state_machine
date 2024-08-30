use super::{state::State, StateMachine};

pub trait SuperState<S: State>: StateMachine<S> + State {}
