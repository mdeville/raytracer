use crate::objects::Object;

struct Node {
    objects: Vec<Box<dyn Object>>,
    children: [Option<Box<Node>>; 8],
}

impl Node {
    fn add_object(&mut self, object: impl Object) {
        todo!()
    }
}
