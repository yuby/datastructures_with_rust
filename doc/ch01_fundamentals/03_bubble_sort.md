좋습니다! 선택 정렬에 대해 잘 이해하신 것 같네요. 그럼 다음 주제로 넘어가볼까요?

다음은 **버블 정렬(Bubble Sort)**에 대해 알아보겠습니다.

**버블 정렬(Bubble Sort)**

버블 정렬은 인접한 두 원소를 검사하여 정렬하는 알고리즘입니다. 인접한 2개의 레코드를 비교하여 크기가 순서대로 되어 있지 않으면 서로 교환합니다. 이 과정을 리스트의 한쪽 끝에서 다른 쪽 끝까지 반복합니다.

버블 정렬의 동작 방식은 다음과 같습니다:

1. 첫 번째 원소부터 인접한 원소끼리 계속 자리를 교환하면서 맨 마지막 자리까지 이동합니다.
2. 한 단계가 끝나면 가장 큰 원소가 맨 뒤로 이동하므로, 맨 뒤의 원소는 정렬에서 제외됩니다.
3. 이 과정을 반복하면 점점 정렬에서 제외되는 데이터가 늘어납니다.

버블 정렬은 구현이 간단하지만, 시간 복잡도가 O(n^2)로 비효율적입니다. 이는 버블 정렬이 대부분의 상황에서 다른 정렬 알고리즘에 비해 느리다는 것을 의미합니다.

이제 Rust로 버블 정렬을 구현해봅시다. "쉬운 코드"로 시작할까요, 아니면 "어려운 코드"로 시작할까요?

```rust
use super::swap;

pub fn run() {
  let mut arr = [7, 4, 5, 1, 6, 9];

  for outer_index in 0..arr.len() {

    for inner_index in 0..(arr.len() - outer_index - 1) {
      let current_val = arr[inner_index];
      let next_val = arr[inner_index + 1];

      if (current_val > next_val) {
        swap::swap(&mut arr, inner_index, inner_index + 1);
        println!("> {:?}", arr);
      }
    }
    println!("==> {:?}", arr);
  }
}
```

네, 버블 정렬 코드를 잘 작성하셨습니다. 코드가 올바르게 동작할 것 같네요. 하지만 몇 가지 개선할 점이 있습니다.

1. `current_val`과 `next_val` 변수는 실제로 필요하지 않습니다. 대신 `arr[inner_index]`와 `arr[inner_index + 1]`를 직접 비교할 수 있습니다.

2. `swap::swap` 함수를 호출하는 대신 `arr.swap(inner_index, inner_index + 1)`을 사용하면 더 간결해집니다.

3. `println!` 문은 디버깅 목적으로는 유용하지만, 실제 버블 정렬 알고리즘에는 필요하지 않습니다.

이러한 점을 고려하여 코드를 다음과 같이 개선할 수 있습니다:

```rust
pub fn run() {
    let mut arr = [7, 4, 5, 1, 6, 9];

    for outer_index in 0..arr.len() {
        for inner_index in 0..(arr.len() - outer_index - 1) {
            if arr[inner_index] > arr[inner_index + 1] {
                arr.swap(inner_index, inner_index + 1);
            }
        }
    }
    
    println!("Sorted array: {:?}", arr);
}
```

이 코드는 여전히 버블 정렬을 정확히 구현하고 있지만, 불필요한 변수 할당과 함수 호출을 제거하여 더 간결해졌습니다.

마지막으로 정렬된 배열을 출력하는 `println!` 문을 추가했습니다.

이 코드를 실행하면 `arr`이 올바르게 정렬된 것을 볼 수 있습니다:

```
Sorted array: [1, 4, 5, 6, 7, 9]
```

잘 작성된 버블 정렬 코드였습니다. 이제 Rust에서 버블 정렬을 어떻게 구현하는지 잘 이해하셨을 거예요.
