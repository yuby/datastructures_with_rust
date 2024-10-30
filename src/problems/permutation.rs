use crate::data_structures::swap::swap;

fn permutation(arr: &mut [i32], offset: usize) {
  if offset + 1 == arr.len() {
    println!("{:?}", arr);
  } else {
    for index in offset..arr.len() {
      swap(arr, offset, index);
      permutation(arr, offset + 1);
      swap(arr, offset, index);
    }
  }
}

pub fn run() {
  let mut arr = [1,2,3];

  permutation(&mut arr, 0);
}


// 시작 배열: [1,2,3]

// 1. offset=0, index=0 시작
//    swap(0,0): [1,2,3]
   
//    1.1 offset=1, index=1 시작
//        swap(1,1): [1,2,3]
       
//        1.1.1 offset=2: 종료조건 도달 
//        출력: [1,2,3]
       
//        swap(1,2): [1,3,2]
       
//        1.1.2 offset=2: 종료조건 도달
//        출력: [1,3,2]
       
//        swap(1,2): [1,2,3] (원복)
   
//    swap(0,0): [1,2,3] (원복)

// 2. offset=0, index=1 시작
//    swap(0,1): [2,1,3]
   
//    2.1 offset=1, index=1 시작
//        swap(1,1): [2,1,3]
       
//        2.1.1 offset=2: 종료조건 도달
//        출력: [2,1,3]
       
//        swap(1,2): [2,3,1]
       
//        2.1.2 offset=2: 종료조건 도달
//        출력: [2,3,1]
       
//        swap(1,2): [2,1,3] (원복)
   
//    swap(0,1): [1,2,3] (원복)

// 3. offset=0, index=2 시작
//    swap(0,2): [3,2,1]
   
//    3.1 offset=1, index=1 시작
//        swap(1,1): [3,2,1]
       
//        3.1.1 offset=2: 종료조건 도달
//        출력: [3,2,1]
       
//        swap(1,2): [3,1,2]
       
//        3.1.2 offset=2: 종료조건 도달
//        출력: [3,1,2]
       
//        swap(1,2): [3,2,1] (원복)
   
//    swap(0,2): [1,2,3] (원복)