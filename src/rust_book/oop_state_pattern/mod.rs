mod oop_way;
mod rust_way;

#[allow(dead_code)]
pub fn run() {
    println!("Do oop originally:");
    let mut post = oop_way::Post::new();
    post.add_text("I ate a salad fot lunch today");
    assert_eq!("",post.content());
    post.request_review();
    post.reject();
    post.request_review();
    assert_eq!("",post.content());
    post.approve();
    assert_eq!("I ate a salad fot lunch today",post.content());
    println!("Do oop rust way:");
    let mut post = rust_way::Post::new();

    post.add_text("I ate a salad for lunch today");

    let post = post.request_review();
    let post = post.reject();
    let post = post.request_review();

    let post = post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());

}