//! Rust for Linux 핵심 기능 학습 라이브러리
//!
//! 이 라이브러리는 Rust로 리눅스 커널을 개발하는데 필요한
//! 핵심 언어 기능들을 학습하기 위한 예제와 유틸리티를 제공합니다.

use std::marker::PhantomPinned;
use std::pin::Pin;

/// 간단한 데이터 구조체 예제
#[derive(Debug, Clone)]
pub struct SimpleData {
    pub id: u32,
    pub value: i32,
    pub name: String,
}

/// Pin을 사용해야 하는 구조체 (자기 참조)
pub struct SelfReferential {
    data: String,
    pointer: *const String,
    _pin: PhantomPinned,
}

impl SelfReferential {
    pub fn new(data: String) -> Pin<Box<Self>> {
        let mut boxed = Box::pin(SelfReferential {
            data,
            pointer: std::ptr::null(),
            _pin: PhantomPinned,
        });

        // SAFETY: 이제 이동하지 않을 것이므로 안전
        let self_ptr: *const String = &boxed.data;
        unsafe {
            let mut_ref = Pin::as_mut(&mut boxed);
            Pin::get_unchecked_mut(mut_ref).pointer = self_ptr;
        }

        boxed
    }

    pub fn get_data(&self) -> &str {
        &self.data
    }
}

/// RCU 스타일 데이터 보호 (간단한 시뮬레이션)
pub struct RcuProtected<T> {
    data: T,
}

impl<T> RcuProtected<T> {
    pub fn new(data: T) -> Self {
        Self { data }
    }

    /// RCU read-side critical section
    pub fn read<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&T) -> R,
    {
        // 실제로는 rcu_read_lock()을 호출
        println!("[RCU] Read lock acquired");
        let result = f(&self.data);
        println!("[RCU] Read lock released");
        result
    }
}

/// 큰 구조체 예제 (스택 오버플로우 유발용)
pub struct LargeStruct {
    // 실제 커널에서는 이런 큰 구조체가 흔함
    pub buffer1: [u8; 1024],
    pub buffer2: [u8; 1024],
    pub buffer3: [u8; 1024],
    pub metadata: [u64; 128],
}

impl Default for LargeStruct {
    fn default() -> Self {
        Self {
            buffer1: [0; 1024],
            buffer2: [0; 1024],
            buffer3: [0; 1024],
            metadata: [0; 128],
        }
    }
}

impl LargeStruct {
    pub fn size_in_bytes() -> usize {
        std::mem::size_of::<Self>()
    }
}

/// 커스텀 스마트 포인터 예제
pub struct CustomPtr<T> {
    inner: Box<T>,
}

impl<T> CustomPtr<T> {
    pub fn new(value: T) -> Self {
        Self {
            inner: Box::new(value),
        }
    }

    pub fn get(&self) -> &T {
        &self.inner
    }
}

// Deref 트레이트 구현으로 자동 역참조 지원
impl<T> std::ops::Deref for CustomPtr<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_data() {
        let data = SimpleData {
            id: 1,
            value: 42,
            name: "test".to_string(),
        };
        assert_eq!(data.id, 1);
        assert_eq!(data.value, 42);
    }

    #[test]
    fn test_self_referential() {
        let s = SelfReferential::new("Hello".to_string());
        assert_eq!(s.get_data(), "Hello");
    }

    #[test]
    fn test_rcu_protected() {
        let rcu = RcuProtected::new(42);
        let value = rcu.read(|data| *data);
        assert_eq!(value, 42);
    }

    #[test]
    fn test_large_struct_size() {
        let size = LargeStruct::size_in_bytes();
        println!("LargeStruct size: {} bytes", size);
        assert!(size >= 4096); // 4KB 이상
    }
}
