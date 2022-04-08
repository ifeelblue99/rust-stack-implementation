struct Stack<T> {
  stack: Vec<T>
}
impl<T> Stack<T>{
  fn new()-> Self{
    Stack{stack: vec![]} 
  }
  fn new_with_capacity(max_size: usize)-> Self{
    Self {
      stack: Vec::with_capacity(max_size)
    }
  }
  fn add(&mut self, num: T){
    self.stack.push(num);
  }
  fn remove(&mut self)-> Option<T>{
    self.stack.pop()
  }
  fn get_size(&mut self) -> usize{
    self.stack.len()
  }
  fn get_last(&mut self) -> Option<&T>{
    self.stack.last()
  }
}

fn main(){
  let mut st:Stack<i8> = Stack{stack: vec![]};
  let mut st2:Stack<i8> = Stack::new_with_capacity(5);
  st2.add(5);
  st2.add(7);
  st.add(1);
  st.add(3);
  println!("last: {:?}", st.get_last());
  println!("size: {}", st.get_size());

  let rm = st.remove();
  match rm {
    Some(val) => println!("removed: {}", val),
    _ => println!("not found")
  }
}
