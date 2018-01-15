extern crate actix;

use actix::{msgs, Actor, Address, Arbiter, Context, System, Handler, ResponseType, ActorFuture, AsyncContext, SyncAddress};
use actix::fut::wrap_future;
use actix::fut;

struct MyActor;

impl Actor for MyActor {
	type Context = Context<Self>;

	fn started(&mut self, ctx: &mut Self::Context) {
		let addr: Address<Self> = ctx.address();
		ctx.spawn(wrap_future(addr.upgrade()).and_then(move |sync_address, actor: &mut MyActor, ctx: &mut Context<MyActor>| {
			sync_address.send(Message);

			//uncommenting the next line "fixes" the issue (prevents sync_address from being dropped)
			//std::mem::forget(sync_address);

			ctx.spawn(wrap_future(addr.upgrade()).and_then(move |sync_address, _, ctx| {
				sync_address.send(Message);
				fut::ok(())
			}).map_err(|_, _, _| ()));

			fut::ok(())
		}).map_err(|_, _, _| ()));
	}
}

struct Message;

impl ResponseType for Message {
	type Item = ();
	type Error = ();
}

impl Handler<Message> for MyActor {
	type Result = ();

	fn handle(&mut self, msg: Message, ctx: &mut Self::Context) -> () {
		println!("Handling message!");
	}
}


fn main() {
	let system = System::new("test");

	let addr: Address<_> = MyActor.start();

	system.run();
}