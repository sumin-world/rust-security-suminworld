# Arbitrary Self Types (임의 자기 타입)

## 개요

Arbitrary Self Types는 메서드의 `self` 매개변수를 다양한 스마트 포인터 타입으로 받을 수 있게 하는 기능입니다.

## 현재 Rust에서 가능한 것

```rust
impl MyStruct {
    fn by_value(self) { }           // ✅ 소유권
    fn by_ref(&self) { }            // ✅ 불변 참조
    fn by_mut(&mut self) { }        // ✅ 가변 참조
    fn by_box(self: Box<Self>) { }  // ✅ Box (특별 케이스)
}
```

## 현재 불가능한 것

```rust
impl MyStruct {
    fn by_rc(self: Rc<Self>) { }           // ❌
    fn by_arc(self: Arc<Self>) { }         // ❌
    fn by_pin(self: Pin<&mut Self>) { }    // ❌
    fn by_custom(self: MyPtr<Self>) { }    // ❌
}
```

## 왜 필요한가?

### 1. 참조 카운팅 포인터

```rust
use std::sync::Arc;

struct Device {
    id: u32,
}

impl Device {
    // ❌ 현재 불가능
    // fn register(self: Arc<Self>) -> Result<(), Error> {
    //     // Arc를 여러 곳에 복사해서 공유
    // }
    
    // ✅ 현재의 해결책 (불편함)
    fn register(arc: Arc<Self>) -> Result<Arc<Self>, Error> {
        // 메서드 체이닝이 자연스럽지 않음
        Ok(arc)
    }
}

let device = Arc::new(Device { id: 1 });

// ❌ 이렇게 하고 싶지만...
// device.register()?;

// ✅ 대신 이렇게 해야 함
let device = Device::register(device)?;
```

### 2. Pin과의 조합

```rust
use std::pin::Pin;

struct AsyncTask {
    state: TaskState,
}

impl AsyncTask {
    // ❌ 현재 불가능
    // fn poll(self: Pin<&mut Self>) -> Poll<Output> {
    //     // Pin 상태 유지하면서 메서드 호출
    // }
    
    // ✅ 현재의 해결책
    fn poll(pinned: Pin<&mut Self>) -> Poll<Output> {
        // ...
    }
}

let mut task = Box::pin(AsyncTask { ... });

// ❌ 이렇게 하고 싶지만...
// task.as_mut().poll();

// ✅ 대신 이렇게
AsyncTask::poll(task.as_mut());
```

### 3. 커널 특유의 패턴

리눅스 커널에서는 복잡한 스마트 포인터가 많습니다:

```rust
// Arc + Pin 조합
type SharedPinned<T> = Pin<Arc<T>>;

struct Driver {
    state: DriverState,
}

impl Driver {
    // 이렇게 하고 싶음!
    fn start(self: SharedPinned<Self>) -> Result<(), Error> {
        // Pin과 Arc의 보장을 모두 유지
    }
}
```

## 기술적 도전 과제

### Deref 체인 문제

Rust는 메서드를 찾을 때 자동으로 역참조를 시도합니다:

```rust
let ptr: Pin<&mut MyStruct> = ...;
ptr.method();  // 어떻게 찾을까?
```

**검색 순서:**
1. `Pin<&mut MyStruct>`에서 `method` 찾기
2. Deref → `&mut MyStruct`에서 찾기
3. Deref → `MyStruct`에서 찾기 ✅

**문제:** 3번 단계에서는 `Pin` 정보가 사라짐!

### 해결책: Receiver Trait

```rust
// 새로운 trait (컴파일러가 특별 취급)
trait Receiver {
    type Target;
}

// 기본 타입들은 자동 구현
impl<T> Receiver for &T { type Target = T; }
impl<T> Receiver for &mut T { type Target = T; }
impl<T> Receiver for Box<T> { type Target = T; }
impl<T> Receiver for Rc<T> { type Target = T; }
impl<T> Receiver for Arc<T> { type Target = T; }
impl<T> Receiver for Pin<P> where P: Receiver { ... }

// 커스텀 포인터도 가능!
impl<T> Receiver for MyPtr<T> {
    type Target = T;
}
```

**검색 순서 개선:**
1. `Receiver` 체인 따라가기 (타입 정보 유지)
2. 실패하면 `Deref` 체인 따라가기 (기존 호환성)

## 실제 사용 예시

### 장치 드라이버

```rust
use std::sync::Arc;

struct NetworkDevice {
    name: String,
    stats: Arc<Stats>,
}

impl NetworkDevice {
    // 미래 문법
    fn register(self: Arc<Self>) -> Result<Arc<Self>, Error> {
        // Arc를 다른 곳에 복사
        register_globally(Arc::clone(&self))?;
        Ok(self)
    }
    
    fn send(self: Arc<Self>, packet: Packet) -> Result<(), Error> {
        // Arc를 유지하면서 비동기 작업 시작
        spawn_async(move || {
            self.hardware_send(packet)
        });
        Ok(())
    }
}

// 사용법
let device = Arc::new(NetworkDevice { ... });
device.register()?.send(packet)?;  // 자연스러운 체이닝!
```

### Future와 Pin

```rust
use std::pin::Pin;
use std::future::Future;

struct MyFuture {
    state: State,
}

impl Future for MyFuture {
    type Output = Result<(), Error>;
    
    // 미래 문법
    fn poll(self: Pin<&mut Self>, cx: &Context) -> Poll<Self::Output> {
        // Pin 보장 유지
        // self가 이미 Pin<&mut Self>이므로 편리
    }
}
```

## 현재 상태

- **시작**: 2024년
- **현재**: 구현 진행 중
- **완료**: 1년 내 가능성 있음
- **추적 이슈**: [Rust GitHub](https://github.com/rust-lang/rust/issues/)

## 마이그레이션 전략

### 단계적 채택

1. **기본 타입부터**
   - `Box`, `Rc`, `Arc` 먼저 지원
   
2. **표준 라이브러리**
   - `Pin`, `RefCell` 등 추가
   
3. **커스텀 타입**
   - 사용자가 `Receiver` 구현

### 하위 호환성

```rust
// 기존 코드는 계속 작동
fn old_style(arc: Arc<T>) { }

// 새 코드는 더 ergonomic
impl T {
    fn new_style(self: Arc<Self>) { }
}
```

## 성능 영향

- **컴파일 타임**: 약간 증가 (타입 검사 복잡도)
- **런타임**: 영향 없음 (zero-cost)
- **바이너리 크기**: 영향 없음

## 관련 기능

### Deref Trait

```rust
trait Deref {
    type Target;
    fn deref(&self) -> &Self::Target;
}
```

Arbitrary Self Types는 Deref와 협력하지만 더 강력합니다.

### 현재의 임시 해결책

```rust
// Extension trait 패턴
trait ArcExt<T> {
    fn my_method(&self);
}

impl<T> ArcExt<T> for Arc<T> {
    fn my_method(&self) {
        // ...
    }
}
```

## 학습 자료

- [RFC 논의](https://github.com/rust-lang/rfcs/)
- [Receiver Trait 설계](https://github.com/rust-lang/rust/)
- [Deref 문서](https://doc.rust-lang.org/std/ops/trait.Deref.html)

## 실습

```bash
cargo run --example smart_pointers
```

이 예제는 다음을 보여줍니다:
- 현재의 제약사항
- 임시 해결책
- 미래 문법 시뮬레이션

## FAQ

**Q: 모든 타입에 적용되나요?**
A: `Receiver` trait을 구현한 타입만 가능합니다.

**Q: 성능 오버헤드는?**
A: 없습니다. 컴파일 타임에 모두 처리됩니다.

**Q: 기존 코드가 깨지나요?**
A: 아니요, 하위 호환됩니다.

**Q: 언제 안정화되나요?**
A: 1년 내 가능성이 높습니다 (2026년 예상).
