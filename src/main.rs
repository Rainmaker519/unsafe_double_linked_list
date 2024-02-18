
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
        head: Node {
            val: 0.0f64,
            next: Option::None,
            prev: Option::None
        },
        tail: &mut Node {
            val: 0.0f64,
            next: Option::None,
            prev: Option::None
        },
    };
    list.add(n1);
    list.add(n2);
    list.add(n3);

    list.set(0, 21.22f64);
    list.set(1, 1f64);
    list.set(2, 11.11f64);

    println!("{:?}",list.get(0));
    println!("{:?}",list.get(1));
    println!("{:?}",list.get(2));
    println!();

    list.remove(1);

    println!("{:?}",list.get(0));
    println!("{:?}",list.get(1));
    println!();

    list.remove(1);

    println!("{:?}",list.get(0));
}


#[derive(Derivative)]
struct DLL {
    nodes: Vec<Node>,
    #[derivative(Default(value = "0"))]
    len: usize,
    head: Node,
    tail: *mut Node,
}
impl DLL {
    fn add(&mut self, mut node: Node) {
        //if there is only one node, tail will stay as none
        if self.len <= 0 {
            self.head = node;
            self.len = 1;
        }
        else {
            let len = self.len;

            if len == 1 {
                let old_last = &mut self.head as *mut Node;
                let new_last = &mut node as *mut Node;

                unsafe {
                    set_next(&mut *old_last, Option::Some(&mut *new_last));
                    set_prev(&mut *new_last, Option::Some(&mut *old_last));
                }

                self.tail = new_last;
                self.len += 1;
            }
            else {
                let old_last = self.tail;
                let new_last = &mut node as *mut Node;

                unsafe {
                    set_next(&mut *old_last, Option::Some(&mut *new_last));
                    set_prev(&mut *new_last, Option::Some(&mut *old_last));

                    self.tail = new_last;
                }

                self.len += 1;
            }
        }
    }

    fn get(&self, ind: usize) -> f64 {
        let mut node_ptr = &self.head;
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

    fn get_mut_ptr(&mut self, ind: usize) -> *mut Node {
        let mut node_ptr = &mut self.head;
        if ind != 0 {
            for i in 0..ind {
                match get_next_mut(node_ptr) {
                    None => { panic!("In DLL get, tried to get next on option::none"); }
                    Some(node) => {
                        node_ptr = node;
                    }
                }
            }
        }
        return node_ptr;
    }

    fn set(&mut self, ind: usize, val: f64) {
        let mut node_ptr = &self.head;
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
            mut_node_ptr = &mut self.head;
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
            unsafe {
                let tail_ref = get_prev_mut(&*self.tail).unwrap();
                set_next(tail_ref,Option::None);
                self.tail = tail_ref as *mut Node;
            }
            self.len -= 1;
            return;
        }
        else if ind == 0 {
            let mut next = get_next_mut(&self.head).unwrap();
            self.head = next.clone();
            set_prev(&mut self.head, Option::None);
            self.len -= 1;
            return;
        }


        let mut node_ptr_pre = self.get_mut_ptr(ind-1);
        unsafe {
            let mut node_ptr_post = self.get_mut_ptr(ind+1);

            let pre = &mut *(node_ptr_pre);
            let post = &mut *(node_ptr_post);
            set_next(pre,Option::Some(post));
            set_prev(post,Option::Some(pre));
        }
        self.len -= 1;

    }
}

#[derive(Clone)]
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
    else {
        let ptr = next.unwrap() as *mut Node;

        unsafe {
            node.next = Option::Some(ptr);
        }
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
    else {
        let ptr = prev.unwrap() as *mut Node;

        unsafe {
            node.prev = Option::Some(ptr);
        }
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
