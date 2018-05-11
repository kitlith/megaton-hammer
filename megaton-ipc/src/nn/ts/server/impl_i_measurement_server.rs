
use megaton_hammer::kernel::{Session, Domain, Object};
#[allow(unused_imports)]
use megaton_hammer::kernel::KObject;
use megaton_hammer::error::*;
use core::ops::{Deref, DerefMut};
use alloc::arc::Arc;

#[derive(Debug)]
pub struct IMeasurementServer<T>(T);

impl IMeasurementServer<Session> {
	pub fn raw_new() -> Result<IMeasurementServer<Session>> {
		use nn::sm::detail::IUserInterface;

		let sm = IUserInterface::new()?;

		let r = sm.get_service(*b"ts\0\0\0\0\0\0").map(|s: KObject| Session::from(s).into());
		if let Ok(service) = r {
			return Ok(service);
		}
		r
	}

	pub fn new() -> Result<Arc<IMeasurementServer<Session>>> {
		use alloc::arc::Weak;
		use spin::Mutex;
		use core::mem::ManuallyDrop;
		lazy_static! {
			static ref HANDLE : Mutex<Weak<IMeasurementServer<Session>>> = Mutex::new(Weak::new());
		}
		if let Some(hnd) = HANDLE.lock().upgrade() {
			return Ok(hnd)
		}

		if let Some(hnd) = ::megaton_hammer::loader::get_override_service(*b"ts\0\0\0\0\0\0") {
			let ret = Arc::new(IMeasurementServer(ManuallyDrop::into_inner(hnd)));
			::core::mem::forget(ret.clone());
			*HANDLE.lock() = Arc::downgrade(&ret);
			return Ok(ret);
		}

		let hnd = Self::raw_new()?;
		let ret = Arc::new(hnd);
		*HANDLE.lock() = Arc::downgrade(&ret);
		Ok(ret)
	}

	pub fn to_domain(self) -> ::core::result::Result<IMeasurementServer<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(IMeasurementServer(domain)),
			Err((sess, err)) => Err((IMeasurementServer(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<IMeasurementServer<Session>> {
		Ok(IMeasurementServer(self.0.duplicate()?))
	}
}

impl<T> Deref for IMeasurementServer<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for IMeasurementServer<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> IMeasurementServer<T> {
	// fn unknown0(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn unknown1(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn unknown2(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn unknown3(&self, UNKNOWN) -> Result<UNKNOWN>;
}

impl<T: Object> From<T> for IMeasurementServer<T> {
	fn from(obj: T) -> IMeasurementServer<T> {
		IMeasurementServer(obj)
	}
}
