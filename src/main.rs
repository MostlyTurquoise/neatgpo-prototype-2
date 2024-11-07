pub mod neatgpo;
use crate::neatgpo::TaskFlow;

fn main() {
    let mut tf = TaskFlow::<dyn Fn()>::new();
    let t1h = tf.register(Box::new(||{
        println!("Task 1");
    }),None).unwrap();
    let t2h = tf.register(Box::new(||{
        println!("Task 2");
    }),Some(&[t1h])).unwrap();
    let t3h = tf.register(Box::new(||{
        println!("Task 3");
    }),Some(&[t1h])).unwrap();
    let t4h = tf.register(Box::new(||{
        println!("Task 4");
    }),Some(&[t2h,t3h])).unwrap();
    tf.execute_from(t4h).expect("Execution failed");
}
