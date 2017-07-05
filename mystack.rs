struct Node {
	item: str,
	bottom: Node,
}

struct Stack {
	head: Node,
	length: i32, 
}

impl Stack {
	fn mknew(&mut self){
		self.head = Node{item: NULL, bottom: NULL};
		self.length = 0;
	}
	fn push(str: val, &mut self){
		let node = Node{item: val, bottom: self.head};
		self.head = node;
	}
	fn pop(&mut self) -> Node {
		let node = self.head;
		self.head = self.head.bottom;
		node.bottom = NULL;
		node
	}
}
