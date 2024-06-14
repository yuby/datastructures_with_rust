좋습니다! 교환(Swap)부터 시작해보겠습니다.

**교환(Swap)**

교환은 두 변수의 값을 서로 바꾸는 연산입니다. 이는 많은 알고리즘에서 기본적인 동작 중 하나입니다.

쉬운 문법의 Rust 코드:

```rust
fn swap(a: &mut i32, b: &mut i32) {
    let temp = *a;
    *a = *b;
    *b = temp;
}

fn main() {
    let mut x = 1;
    let mut y = 2;
    println!("Before swap: x = {}, y = {}", x, y);
    swap(&mut x, &mut y);
    println!("After swap: x = {}, y = {}", x, y);
}
```

이 코드에서 `swap` 함수는 두 개의 `i32` 변수에 대한 가변 참조를 받아 값을 교환합니다. `main` 함수에서는 `x`와 `y`라는 두 개의 가변 변수를 선언하고, `swap` 함수를 호출할 때 이들의 가변 참조를 전달합니다.

고급 문법의 Rust 코드:

```rust
fn main() {
    let mut x = 1;
    let mut y = 2;
    println!("Before swap: x = {}, y = {}", x, y);
    std::mem::swap(&mut x, &mut y);
    println!("After swap: x = {}, y = {}", x, y);
}
```

Rust 표준 라이브러리에는 `std::mem::swap` 함수가 있습니다. 이 함수는 제네릭하게 구현되어 있어 모든 타입에 대해 작동합니다. 이를 사용하면 직접 `swap` 함수를 구현할 필요가 없습니다.

이 내용에 대해 질문이나 토의할 점이 있나요? 없다면 다음 주제인 선택 정렬로 넘어가겠습니다.

```rust
fn swap(a: &mut i32, b: &mut i32) {
  let temp = *a;

  *a = *b;
  *b = temp;
}

pub fn run() {
  let mut arr = [1, 2, 3, 4, 5];
  swap(&mut arr[1], &mut arr[2]);

  println!("{:?}", arr);
}
```

-> Rust의 소유권 시스템 때문에 이런 오류가 발생한 것 같네요. 동일한 데이터에 대해 여러 개의 가변 참조를 만들 수 없습니다.

아, 그렇군요. 죄송합니다. 배열 전체를 전달하지 않고 두 요소의 참조만 전달하여 교환하는 것은 불가능합니다.

Rust에서 참조는 원래 값에 대한 별명(alias)일 뿐입니다. 참조를 서로 교환한다고 해서 원래 값이 교환되지는 않습니다.

다음 코드를 보시면 이해가 될 것입니다:

```rust
fn swap(a: &mut i32, b: &mut i32) {
    let temp = a;
    a = b;
    b = temp;
}

fn main() {
    let mut arr = [1, 2, 3, 4, 5];
    swap(&mut arr[1], &mut arr[2]);
    println!("{:?}", arr);
}
```

이 코드에서 `swap` 함수는 `a`와 `b`라는 두 개의 `i32` 참조를 받습니다. 함수 내에서 이 참조들을 교환하려고 시도합니다. 하지만 이는 `a`와 `b`가 가리키는 원래 값을 바꾸지 않습니다. 대신 `a`와 `b`라는 지역 변수만 교환할 뿐입니다.

이 코드를 실행하면 다음 결과가 출력됩니다:

```
[1, 2, 3, 4, 5]
```

배열이 변경되지 않은 것을 볼 수 있습니다.

따라서 두 값을 교환하려면 값 자체에 접근해야 합니다. 이를 위해서는 배열에 대한 가변 참조를 전달하고, 해당 인덱스의 값을 직접 교환해야 합니다.

```rust
fn swap<T>(arr: &mut [T], i: usize, j: usize) {
    let temp = arr[i];
    arr[i] = arr[j];
    arr[j] = temp;
}
```

이것이 Rust에서 값을 교환하는 올바른 방법입니다. 참조만 전달해서는 원래 값을 변경할 수 없습니다.


네, 이해했습니다. 그럼 `split_at_mut`을 사용하지 않고 쉬운 문법으로 작성해보겠습니다.

```rust
fn swap(arr: &mut [i32], i: usize, j: usize) {
    let temp = arr[i];
    arr[i] = arr[j];
    arr[j] = temp;
}

fn main() {
    let mut arr = [1, 2, 3, 4, 5];
    swap(&mut arr, 1, 2);
    println!("{:?}", arr);
}
```

이 버전에서는 `swap` 함수가 배열에 대한 가변 참조와 두 인덱스를 받습니다. 임시 변수 `temp`를 사용하여 `arr[i]`의 값을 저장합니다. 그런 다음 `arr[i]`에 `arr[j]`의 값을 할당하고, `arr[j]`에 `temp`의 값을 할당합니다. 이렇게 하면 두 요소의 값이 교환됩니다.

`main` 함수에서는 `swap` 함수를 호출할 때 `&mut arr`을 전달하여 배열 전체에 대한 가변 참조를 제공합니다. 이렇게 하면 `swap` 함수 내에서 배열의 요소에 접근하여 수정할 수 있습니다.

이 코드는 이전 버전과 동일한 결과를 생성할 것입니다:

```
[1, 3, 2, 4, 5]
```

이제 더 쉬운 문법으로 배열 요소를 교환할 수 있습니다. 더 궁금한 점이 있나요?