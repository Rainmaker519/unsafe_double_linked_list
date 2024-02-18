
use std::slice;
use derivative::Derivative;


fn main() {
    let mut n1 = Node {
        val: 0.1f64,
        next: Option::None,
        prev: Option::None,
    };
    let mut n2 = Node {
        val: 0.2f64,
        next: Option::None,
        prev: Option::None,
    };
    let mut n3 = Node {
        val: 0.3f64,
        next: Option::None,
        prev: Option::None,
    };

    let mut list = DLL {
        nodes: vec![],
        len: 0,
    };
    list.add(n1);
    list.add(n2);
    list.add(n3);

    list.set(0, 21.22f64);
    list.set(01, 1f64);
    list.set(2, 11.11f64);

    println!("{:?}",list.get(0));
    println!("{:?}",list.get(1));
    println!("{:?}",list.get(2));

    list.remove(1);

    println!("{:?}",list.get(0));
    println!("{:?}",list.get(1));
}


#[derive(Derivative)]
struct DLL {
    nodes: Vec<Node>,
    #[derivative(Default(value = "0"))]
    len: usize,
}
impl DLL {
    fn add(&mut self, node: Node) {
        if self.len <= 0 {
            self.nodes.push(node);
            self.len = 1;
        }
        else {
            let len = self.len;

            if len != self.nodes.len() {
                panic!("Len incorrect.");
            }

            self.nodes.push(node);

            let old_last = &mut self.nodes[len-1] as *mut Node;
            let new_last = &mut self.nodes[len] as *mut Node;

            unsafe {
                set_next(&mut *old_last, Option::Some(&mut *new_last));
                set_prev(&mut *new_last, Option::Some(&mut *old_last));
            }

            self.len += 1;
        }
    }

    fn get(&self, ind: usize) -> f64 {
        let mut node_ptr = &self.nodes[0];
        if ind != 0 {
            for i in 0..ind {
                match get_next(node_ptr) {
                    None => { panic!("In DLL get, tried to get next on option::none"); }
                    Some(node) => {
                        node_ptr = node;
                    }
                }
            }
        }
        return get_value(node_ptr);
    }

    fn set(&mut self, ind: usize, val: f64) {
        let mut node_ptr = &self.nodes[0];
        let mut_node_ptr: &mut Node;

        if ind > 1 {
            for i in 0..ind - 1 {
                match get_next(node_ptr) {
                    None => { panic!("In DLL set, tried to get next on option::none"); }
                    Some(node) => {
                        node_ptr = node;
                    }
                }
            }
        }
        if ind == 0 {
            mut_node_ptr = &mut self.nodes[0];
        }
        else {
            match get_next_mut(node_ptr) {
                None => {panic!("In DLL set, tried to get next on option::none");}
                Some(node) => {
                    mut_node_ptr = node;
                }
            }
        }

        set_value(mut_node_ptr, val);
    }

    fn remove(&mut self, ind: usize) {
        //mostly working, probably some edge cases where the unsafe stuff is actually unsafe

        if ind < 0 || ind > self.len-1 || self.len < 1 {
            panic!("Remove ind not in range of list it's used on!");
        }

        if ind == self.len-1 {
            set_next(&mut self.nodes[ind-1], Option::None);
            self.nodes.remove(ind);
            return;
        }
        else if ind == 0 {
            set_prev(&mut self.nodes[1], Option::None);
            self.nodes.remove(0);
            return;
        }

        let mut node_ptr_pre = &mut self.nodes[ind-1] as *mut Node;
        let mut node_ptr_post = &mut self.nodes[ind+1] as *mut Node;
        self.nodes.remove(ind);
        unsafe {
            let pre = &mut *(node_ptr_pre);
            let post = &mut *(node_ptr_post);
            set_next(pre,Option::Some(post));
            set_prev(post,Option::Some(pre));
        }

    }
}

struct Node {
    val: f64,
    next: Option<*mut Node>,
    prev: Option<*mut Node>,
}

fn get_next(node: &Node) -> Option<&Node> {
    let none = node.next.is_none();

    if none {
        return Option::None;
    }

    unsafe {
        return Option::Some(
            &mut *(node.next.unwrap())
        )
    }
}
fn get_next_mut(node: &Node) -> Option<&mut Node> {
    let none = node.next.is_none();

    if none {
        return Option::None;
    }

    unsafe {
        return Option::Some(
            &mut *(node.next.unwrap())
        )
    }
}
fn set_next(node: &mut Node, next: Option<&mut Node>) {
    let none = next.is_none();

    if none {
        node.next = Option::None;
    }
    let ptr = next.unwrap() as *mut Node;

    unsafe {
        node.next = Option::Some(ptr);
    }
}
fn get_prev(node: &Node) -> Option<&Node> {
    let none = node.prev.is_none();

    if none {
        return Option::None;
    }

    unsafe {
        return Option::Some(
            &mut *(node.prev.unwrap())
        )
    }
}
fn get_prev_mut(node: &Node) -> Option<&mut Node> {
    let none = node.prev.is_none();

    if none {
        return Option::None;
    }

    unsafe {
        return Option::Some(
            &mut *(node.prev.unwrap())
        )
    }
}
fn set_prev(node: &mut Node, prev: Option<&mut Node>) {
    let none = prev.is_none();

    if none {
        node.prev = Option::None;
    }
    let ptr = prev.unwrap() as *mut Node;

    unsafe {
        node.prev = Option::Some(ptr);
    }
}
fn get_value(node: &Node) -> f64 {
    node.val
}
fn set_value(node: &mut Node, value: f64) {
    node.val = value;
}



/*
fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();

    assert!(mid <= len);

    (&mut values[..mid], &mut values[mid..])
}

fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr();

    //since slices are really just a pointer to some data
    // and a length, ptr and len is really the same thing
    // just without a layer of safety through access restriction

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr,mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )

    }
}
*/
