use blog::Post;

fn main() {
    let mut post = Post::new();
    post.add_text("I ate a salad for lunch today");
    println!("{:?}",post);
    assert_eq!("",post.content());

    post.request_review();
    assert_eq!("abc",post.content());

//    post.approve();
//    assert_eq!("I ate a salad for lunch today",post.content);

}
