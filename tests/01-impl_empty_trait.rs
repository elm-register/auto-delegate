use macros::delegate_trait;

#[delegate_trait]
trait Nothing {}

#[allow(dead_code)]
struct Object;

fn main() {
    impl_delegate_nothing!(Object);
}
