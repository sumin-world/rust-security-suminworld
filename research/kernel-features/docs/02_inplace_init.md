# In-place Initialization (제자리 초기화)

## 개요

In-place initialization은 객체를 처음부터 최종 목적지(예: 힙)에 생성하는 것을 말합니다. 현재 Rust는 스택에서 생성 후 힙으로 이동합니다.

## 문제 상황

### 현재 Rust의 동작

```rust
let large = LargeStruct { /* ... */ };  // 1. 스택에 생성
let boxed = Box::new(large);            // 2. 힙으로 복사
```

**문제점:**
1. 스택 공간 낭비
2. 복사 오버헤드
3. 스택 오버플로우 위험

### 커널의 특수성

일반 프로그램:
- 스택 크기: ~8MB (충분히 큼)

리눅스 커널:
- 스택 크기: 8-16KB (매우 작음!)
- 이유: 각 스레드마다 스택이 필요하고, 수천 개의 스레드가 있을 수 있음

```rust
// ❌ 이런 코드는 커널에서 재앙!
struct DriverState {
    buffer1: [u8; 4096],  // 4KB
    buffer2: [u8; 4096],  // 4KB
    // 총 8KB - 커널 스택 전체를 차지!
}

let state = DriverState { ... };  // 스택 오버플로우!
```

## 실제 사례: Asahi GPU 드라이버

Apple Silicon GPU 드라이버 개발 중 발견된 문제:

```rust
struct GpuState {
    field1: Type1,
    field2: Type2,
    // ... 수백 개의 필드
    field500: Type500,
}
// 총 크기: 수십 KB!
```

**해결책:**
- 커스텀 매크로 개발
- pin-init 크레이트 사용

## 제안된 해결책

### 방안 1: `init` 키워드

```rust
// Alice Ryhl & Benno Lossin 제안
let boxed = Box::init MyStruct {
    field1: value1,
    field2: value2,
};
```

**장점:**
- 간단하고 직관적
- 최소한의 언어 변경

**단점:**
- PinInit trait에 고정됨

### 방안 2: `&out` 참조

```rust
// Taylor Cramer 제안
fn initialize(dest: &out MyStruct) {
    dest.field1 = value1;  // 쓰기만 가능
    dest.field2 = value2;
}

let boxed = Box::new_uninit();
initialize(boxed.as_out());
let boxed = boxed.assume_init();
```

**장점:**
- 매우 유연함
- 부분 초기화 추적 가능

**단점:**
- 복잡한 구현
- 타입 시스템 변경 필요

### 방안 3: 보장된 최적화

```rust
// C++ placement new와 유사
let boxed = Box::new(MyStruct { ... });
// 컴파일러가 자동으로 힙에 직접 생성
```

**장점:**
- 기존 코드와 호환

**단점:**
- 보장되지 않으면 문제

## Pin과의 조합

커널에서는 Pin + 제자리 초기화가 자주 필요합니다:

```rust
// 현재: pin_init!() 매크로
let pinned = pin_init!(MyStruct {
    field1: value1,
    field2: value2,
});

// 미래: 언어 기능
let pinned = Pin::new(Box::init MyStruct {
    field1: value1,
    field2: value2,
});
```

## 성능 비교

### 스택 vs 힙 복사

```
작은 구조체 (8 bytes):
  스택 → 힙: 무시 가능한 오버헤드

중간 구조체 (1KB):
  스택 → 힙: ~10% 오버헤드

큰 구조체 (8KB+):
  스택 → 힙: 상당한 오버헤드 + 스택 오버플로우 위험!
```

### 벤치마크 예시

```bash
cargo run --example inplace_init
```

출력 예시:
```
작은 구조체: 5μs
중간 구조체: 50μs
큰 구조체: 500μs (+ 스택 오버플로우 위험)
```

## 관련 크레이트

### pin-init

Rust for Linux 프로젝트에서 만든 임시 해결책:

```rust
use pin_init::*;

#[pin_data]
struct MyStruct {
    field1: i32,
    #[pin]
    field2: String,
}

let pinned = pin_init!(MyStruct {
    field1: 42,
    field2: "hello".to_string(),
});
```

링크: https://github.com/Rust-for-Linux/pin-init

## 현재 상태

- **현재**: 여러 제안 검토 중
- **구현**: 아직 시작 안 됨
- **타임라인**: 미정

## 학습 자료

- [HackMD 문서](https://hackmd.io/@rust-lang-team/r1zNmpwwgl#In-place-initialization-via-outptrs)
- [pin-init 크레이트](https://github.com/Rust-for-Linux/pin-init)
- [Box::new_uninit() 문서](https://doc.rust-lang.org/std/boxed/struct.Box.html#method.new_uninit)

## 실습

```bash
cargo run --example inplace_init
```

이 예제는 다음을 보여줍니다:
- 현재의 문제점
- 스택 오버플로우 시연
- 해결책 비교
