
use megaton_hammer::kernel::{KObject, Session, Domain, Object};
use megaton_hammer::error::*;
use core::ops::{Deref, DerefMut};
use alloc::arc::Arc;

#[derive(Debug)]
pub struct IRandomInterface<T>(T);

impl IRandomInterface<Session> {
	pub fn new() -> Result<Arc<IRandomInterface<Session>>> {
		use alloc::arc::Weak;
		use spin::Mutex;
		use core::mem::ManuallyDrop;
		lazy_static! {
			static ref HANDLE : Mutex<Weak<IRandomInterface<Session>>> = Mutex::new(Weak::new());
		}
		if let Some(hnd) = HANDLE.lock().upgrade() {
			return Ok(hnd)
		}

		use nn::sm::detail::IUserInterface;

		let sm = IUserInterface::new()?;

		if let Some(hnd) = ::megaton_hammer::loader::get_override_service(*b"csrng\0\0\0") {
			let ret = Arc::new(IRandomInterface(ManuallyDrop::into_inner(hnd)));
			::core::mem::forget(ret.clone());
			*HANDLE.lock() = Arc::downgrade(&ret);
			return Ok(ret);
		}

		let r = sm.get_service(*b"csrng\0\0\0").map(|s: KObject| Arc::new(Session::from(s).into()));
		if let Ok(service) = r {
			*HANDLE.lock() = Arc::downgrade(&service);
			return Ok(service);
		}
		r
	}

	pub fn to_domain(self) -> ::core::result::Result<IRandomInterface<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(IRandomInterface(domain)),
			Err((sess, err)) => Err((IRandomInterface(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<IRandomInterface<Session>> {
		Ok(IRandomInterface(self.0.duplicate()?))
	}
}

impl<T> Deref for IRandomInterface<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for IRandomInterface<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> IRandomInterface<T> {
	// fn get_random_bytes(&self, UNKNOWN) -> Result<UNKNOWN>;
}

impl<T: Object> From<T> for IRandomInterface<T> {
	fn from(obj: T) -> IRandomInterface<T> {
		IRandomInterface(obj)
	}
}
