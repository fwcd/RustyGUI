use std::marker::PhantomData;

pub trait Listener<E> {
	fn fire(&mut self, event: E);
}

pub struct ListenerList<'a, E> {
	listeners: Vec<Box<Listener<E> + 'a>>
}

impl <'a, E: 'a> ListenerList<'a, E> {
	pub fn new() -> ListenerList<'a, E> {
		ListenerList { listeners: Vec::new() }
	}
	
	pub fn add(&mut self, listener: Box<Listener<E>>) {
		self.listeners.push(listener);
	}
	
	pub fn add_listener<T: 'a>(&mut self, listener: T) where T: FnMut(E) {
		self.listeners.push(Box::new(LambdaListener::of(listener)));
	}
}

pub struct LambdaListener<E, T> where T: FnMut(E) {
	listener: T,
	phantom: PhantomData<E>
}

impl <E, T: FnMut(E)> LambdaListener<E, T> {
	pub fn of(listener: T) -> LambdaListener<E, T> {
		LambdaListener { listener: listener, phantom: PhantomData }
	}
}

impl <E, T: FnMut(E)> Listener<E> for LambdaListener<E, T> {
	fn fire(&mut self, event: E) {
		(self.listener)(event);
	}
}
