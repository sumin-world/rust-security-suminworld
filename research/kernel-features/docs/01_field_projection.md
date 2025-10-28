# Field Projections (필드 투영)

## 개요

Field Projection은 구조체의 포인터를 그 구조체 내부 필드의 포인터로 변환하는 작업을 말합니다.

## 왜 필요한가?

### 기본 참조는 이미 가능

```rust
struct MyStruct {
    field1: i32,
    field2: String,
}

fn project(s: &MyStruct) -> &i32 {
    &s.field1  // ✅ 간단!
}
```

### 문제: 커스텀 스마트 포인터에서는?

```rust
struct CustomPtr<T> {
    inner: Box<T>,
}

// ❌ 이게 안 됨!
// fn project(ptr: CustomPtr<MyStruct>) -> CustomPtr<i32> { ... }
```

## 리눅스 커널에서 왜 중요한가?

### 1. RCU (Read-Copy-Update) 패턴

리눅스 커널에서 매우 흔한 동기화 패턴:

```rust
struct Data {
    mutex: Mutex,
    frequently_read: i32,    // RCU로 보호
    rarely_modified: String, // Mutex로 보호
}
```

**C에서의 사용법:**
```c
// Reader (빠름!)
rcu_read_lock();
int value = data->frequently_read;
rcu_read_unlock();

// Writer (느림)
mutex_lock(&data->mutex);
data->rarely_modified = "new value";
mutex_unlock(&data->mutex);
```

**현재 Rust의 문제:**
```rust
// ❌ Mutex 전체를 잠궈야 함
let data = mutex.lock();
let value = data.frequently_read;  // RCU 필드만 읽는데도!
```

**Field Projection이 있다면:**
```rust
// ✅ Mutex 잠금 없이 RCU 필드 접근!
let rcu_field: &Rcu<i32> = mutex.project_field();
let value = rcu_field.read();  // 빠름!
```

### 2. Pin과의 조합

커널에서는 메모리에 고정된 객체가 많습니다 (DMA 버퍼, 자기 참조 구조체 등).

```rust
struct DeviceBuffer {
    movable_metadata: u32,      // 이동 가능
    pinned_dma_buffer: [u8; 4096], // 이동 불가 (하드웨어가 주소 참조)
}
```

**현재: 각 필드마다 unsafe 코드 작성**
```rust
impl DeviceBuffer {
    fn get_metadata(self: Pin<&mut Self>) -> &mut u32 {
        unsafe {
            &mut Pin::get_unchecked_mut(self).movable_metadata
        }
    }
    
    fn get_dma_buffer(self: Pin<&mut Self>) -> Pin<&mut [u8; 4096]> {
        unsafe {
            Pin::new_unchecked(&mut Pin::get_unchecked_mut(self).pinned_dma_buffer)
        }
    }
}
```

**Field Projection 후: 자동!**
```rust
let pinned: Pin<&mut DeviceBuffer> = ...;
let metadata = pinned.movable_metadata;    // 자동으로 &mut u32
let dma = pinned.pinned_dma_buffer;        // 자동으로 Pin<&mut [u8]>
```

## 타입 시그니처

일반화된 field projection의 시그니처:

```rust
Container<'a, Struct> -> Output<'a, Field>
```

예시:
- `&MyStruct` → `&Field`
- `&mut MyStruct` → `&mut Field`
- `Pin<&mut MyStruct>` → `Pin<&mut Field>` (필드가 !Unpin이면)
- `Pin<&mut MyStruct>` → `&mut Field` (필드가 Unpin이면)
- `Arc<MyStruct>` → `Arc<Field>` (미래)

## 현재 상태

- **시작**: 2022년 Kangrejos
- **현재**: 설계 단계
- **추적**: https://github.com/rust-lang/rust/pull/146307
- **목표**: 2027년 Debian 14

## 학습 자료

- [RFC 초안](https://github.com/rust-lang/rfcs/)
- [pin-project 크레이트](https://docs.rs/pin-project/) - 현재의 임시 해결책
- [Pin 문서](https://doc.rust-lang.org/std/pin/)

## 실습

`examples/field_projection.rs`를 실행해보세요:

```bash
cargo run --example field_projection
```
