struct Node<'a, T> {
    next: Option<&'a Node<'a, T>>,
    previous: Option<&'a Node<'a, T>>,
    value: T,
}

struct LinkedList<'a, T> {
    head: Option<&'a Node<'a, T>>,
    tail: Option<&'a Node<'a, T>>,
}

impl<'a, T> LinkedList<'a, T> {
    fn new() -> Self {
        Self {
            head: None,
            tail: None,
        }
    }

    fn push(&mut self, item: T) {
        if let Some(mut head) = self.head {}
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_nodes() {
        let mut linked_list = LinkedList::new();
        linked_list.push(55);
    }
}
