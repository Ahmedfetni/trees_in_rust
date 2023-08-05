//generic binary tree
mod binary_trees{
    pub struct Node<T>{
        value:T,
        left: Option<Node<T>>,
        right: Option<Node<T>>

    }
    impl <T>  Node<T> {
        pub fn new(value:T) -> Self{
            Node{
                value,
                left:None,
                right:None
            }
        }
        pub fn add_on_the_right(&self,value:T){
            match self.right {
                None =>  self.right = Node<T>::new(value),
                Some(node)=> node.add_on_the_right(value)
            }
        }

        pub fn add_on_the_left(&self,value:T){
            match self.right {
                None =>  self.left = Node<T>::new(value),
                Some(node)=> node.add_on_the_left(value)
            }
        }

        pub fn del(&self,value:T) -> Option<T>{
                if self.value == value {
                    return value 
                }
        }
    }

    
}

//
mod trees{
    pub struct Node<T>{
        value:T,
        children:Vec<Node<T>>,
    }
    impl <T> Node<T>{
         fn new(value:T){
            Node{
                value,
                children:Vec<Node<T>>::new()
            }
        }
    }


}