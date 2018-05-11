
use megaton_hammer::kernel::{Session, Domain, Object};
#[allow(unused_imports)]
use megaton_hammer::kernel::KObject;
use megaton_hammer::error::*;
use core::ops::{Deref, DerefMut};
use alloc::arc::Arc;

#[derive(Debug)]
pub struct IReader<T>(T);

impl IReader<Session> {
	pub fn raw_new() -> Result<IReader<Session>> {
		use nn::sm::detail::IUserInterface;

		let sm = IUserInterface::new()?;

		let r = sm.get_service(*b"arp:r\0\0\0").map(|s: KObject| Session::from(s).into());
		if let Ok(service) = r {
			return Ok(service);
		}
		r
	}

	pub fn new() -> Result<Arc<IReader<Session>>> {
		use alloc::arc::Weak;
		use spin::Mutex;
		use core::mem::ManuallyDrop;
		lazy_static! {
			static ref HANDLE : Mutex<Weak<IReader<Session>>> = Mutex::new(Weak::new());
		}
		if let Some(hnd) = HANDLE.lock().upgrade() {
			return Ok(hnd)
		}

		if let Some(hnd) = ::megaton_hammer::loader::get_override_service(*b"arp:r\0\0\0") {
			let ret = Arc::new(IReader(ManuallyDrop::into_inner(hnd)));
			::core::mem::forget(ret.clone());
			*HANDLE.lock() = Arc::downgrade(&ret);
			return Ok(ret);
		}

		let hnd = Self::raw_new()?;
		let ret = Arc::new(hnd);
		*HANDLE.lock() = Arc::downgrade(&ret);
		Ok(ret)
	}

	pub fn to_domain(self) -> ::core::result::Result<IReader<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(IReader(domain)),
			Err((sess, err)) => Err((IReader(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<IReader<Session>> {
		Ok(IReader(self.0.duplicate()?))
	}
}

impl<T> Deref for IReader<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for IReader<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> IReader<T> {
	// fn read_header0(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn read_header1(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn read_data0(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn read_data1(&self, UNKNOWN) -> Result<UNKNOWN>;
}

impl<T: Object> From<T> for IReader<T> {
	fn from(obj: T) -> IReader<T> {
		IReader(obj)
	}
}
