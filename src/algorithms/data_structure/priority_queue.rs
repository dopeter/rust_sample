

#[derive(Debug)]
struct PriorityQueue<T> where T: PartialOrd + Clone {
    pq:Vec<T>
}

impl<T> PriorityQueue<T> where T: PartialOrd + Clone{

    pub fn new()->PriorityQueue<T>{
        PriorityQueue{pq:Vec::new()}
    }

    pub fn len(&self)->usize{
        self.pq.len()
    }

    pub fn is_empty(&self)->bool{
        self.pq.len()==0
    }

    pub fn insert(&mut self,value:T){
        self.pq.push(value)
    }

    pub fn max(&self)->Option<T>{
        if self.is_empty(){
            return None
        }

        let max=self.max_index();
        Some(self.pq[max].clone())

    }

    pub fn min(&self)->Option<T>{
        if self.is_empty(){
            return None
        }

        let min=self.min_index();
        Some(self.pq[min].clone())
    }

    pub fn delete_max(&mut self)->Option<T>{
        if self.is_empty(){
            return None
        }

        let max=self.max_index();
        Some(self.pq.remove(max).clone())

    }

    pub fn delete_min(&mut self)-> Option<T>{
        if self.is_empty(){
            return None
        }

        let min=self.min_index();
        Some(self.pq.remove(min).clone())

    }

    pub fn max_index(&self)->usize{
        let mut max=0;
        for index in 1..self.pq.len()-1{
            if self.pq[max]<self.pq[index]{
                max=index
            }
        }

        max
    }

    pub fn min_index(&self)->usize{
        let mut min=0;

        for index in 1..self.pq.len()-1{
            if self.pq[min]>self.pq[index]{
                min=index
            }
        }

        min
    }

}

pub fn test_keep_min(){
    let mut pq=PriorityQueue::new();

    pq.insert(3);
    pq.insert(2);
    pq.insert(1);
    pq.insert(4);
    assert_eq!(pq.min().unwrap(), 1);

}

pub fn test_keep_max() {
    let mut pq = PriorityQueue::new();
    pq.insert(2);
    pq.insert(4);
    pq.insert(1);
    pq.insert(3);
    assert!(pq.max().unwrap() == 4);
}

pub fn test_is_empty() {
    let mut pq = PriorityQueue::new();
    assert!(pq.is_empty());
    pq.insert(1);
    assert!(!pq.is_empty());
}

pub fn test_len() {
    let mut pq = PriorityQueue::new();
    assert!(pq.len() == 0);
    pq.insert(2);
    pq.insert(4);
    pq.insert(1);
    assert!(pq.len() == 3);
}

pub fn test_delete_min() {
    let mut pq = PriorityQueue::new();
    pq.insert(3);
    pq.insert(2);
    pq.insert(1);
    pq.insert(4);
    assert!(pq.len() == 4);
    assert!(pq.delete_min().unwrap() == 1);
    assert!(pq.len() == 3);
}

pub fn test_delete_max() {
    let mut pq = PriorityQueue::new();
    pq.insert(2);
    pq.insert(10);
    pq.insert(1);
    pq.insert(6);
    pq.insert(3);
    assert!(pq.len() == 5);
    assert!(pq.delete_max().unwrap() == 10);
    assert!(pq.len() == 4);
}

pub fn priority_queue_test_aggregate() {
    test_len();
    test_delete_max();
    test_delete_min();
    test_is_empty();
    test_keep_max();
    test_keep_min();
}