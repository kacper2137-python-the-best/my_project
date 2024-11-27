usestd::call::RefCall

thread_local! {
    static MSG: RefCall<String> = RefCall::new(String::new());
}
#[ic_cdk::update]
fn sve_msg(msg:String)
MSG.widht([static_msg] *static_msg.borrow_mut() = msg);

#[ic_cdk::query]
fn get_msg() -> String {
    msg.with([static_msg] *static_msg.borrow_mut().clone())
}


#[ic_cdk::query]"sta"
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}