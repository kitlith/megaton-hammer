
use megaton_hammer::kernel::{Session, Domain, Object};
#[allow(unused_imports)]
use megaton_hammer::kernel::KObject;
use megaton_hammer::error::*;
use core::ops::{Deref, DerefMut};
use alloc::arc::Arc;

#[derive(Debug)]
pub struct IDevelopInterface<T>(T);

impl IDevelopInterface<Session> {
	pub fn raw_new() -> Result<IDevelopInterface<Session>> {
		use nn::sm::detail::IUserInterface;

		let sm = IUserInterface::raw_new()?;

		let session = sm.get_service(*b"ns:dev\0\0")?;
		let object : Self = Session::from(session).into();
		Ok(object)
	}

	pub fn new() -> Result<Arc<IDevelopInterface<Session>>> {
		use alloc::arc::Weak;
		use spin::Mutex;
		use core::mem::ManuallyDrop;
		lazy_static! {
			static ref HANDLE : Mutex<Weak<IDevelopInterface<Session>>> = Mutex::new(Weak::new());
		}
		if let Some(hnd) = HANDLE.lock().upgrade() {
			return Ok(hnd)
		}

		if let Some(hnd) = ::megaton_hammer::loader::get_override_service(*b"ns:dev\0\0") {
			let ret = Arc::new(IDevelopInterface(ManuallyDrop::into_inner(hnd)));
			::core::mem::forget(ret.clone());
			*HANDLE.lock() = Arc::downgrade(&ret);
			return Ok(ret);
		}

		let hnd = Self::raw_new()?;
		let ret = Arc::new(hnd);
		*HANDLE.lock() = Arc::downgrade(&ret);
		Ok(ret)
	}

	pub fn to_domain(self) -> ::core::result::Result<IDevelopInterface<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(IDevelopInterface(domain)),
			Err((sess, err)) => Err((IDevelopInterface(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<IDevelopInterface<Session>> {
		Ok(IDevelopInterface(self.0.duplicate()?))
	}
}

impl<T> Deref for IDevelopInterface<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for IDevelopInterface<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> IDevelopInterface<T> {
	// fn launch_title(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn terminate_title_by_pid(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn terminate_title_by_title_id(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn get_ns_dev_wait_event(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(3)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn get_ns_dev_event_type(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn terminate_crashing_title(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn install_title(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(6)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn set_event_state6(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn set_event_state(&self, UNKNOWN) -> Result<UNKNOWN>;
}

impl<T: Object> From<T> for IDevelopInterface<T> {
	fn from(obj: T) -> IDevelopInterface<T> {
		IDevelopInterface(obj)
	}
}
