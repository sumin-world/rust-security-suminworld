//! Field Projection (필드 투영) 예제
//! 
//! 이 예제는 구조체의 포인터를 필드 포인터로 변환하는 과정을 보여줍니다.
//! 현재 Rust에서 가능한 것과 불가능한 것을 비교해봅니다.

use std::pin::Pin;

/// 복잡한 데이터 구조체
#[derive(Debug)]
struct ComplexData {
    id: u32,
    name: String,
    config: Config,
}

#[derive(Debug)]
struct Config {
    enabled: bool,
    timeout: u64,
}

/// ✅ 기본 참조는 필드 투영이 쉬움
fn project_to_id(data: &ComplexData) -> &u32 {
    &data.id
}

fn project_to_config(data: &ComplexData) -> &Config {
    &data.config
}

/// ✅ 가변 참조도 마찬가지
fn project_to_name_mut(data: &mut ComplexData) -> &mut String {
    &mut data.name
}

/// ✅ 원시 포인터도 가능하지만 unsafe
unsafe fn project_raw_pointer(ptr: *mut ComplexData) -> *mut u32 {
    // C에서 하는 것과 동일: &(ptr->id)
    unsafe { &raw mut (*ptr).id }
}

/// 🚧 문제: Pin<&mut T>에서 필드로의 투영
/// 
/// Pin은 메모리에서 이동 불가능한 타입을 표시합니다.
/// 필드를 투영할 때 어떤 필드는 Pin이 필요하고, 어떤 필드는 필요 없을 수 있습니다.
struct PinnedData {
    movable_field: i32,      // Unpin: 이동 가능
    unmovable_field: String, // 이동 불가능하다고 가정
}

/// 현재 Rust에서 Pin 투영을 하려면 수동으로 unsafe 코드 작성 필요
fn project_pinned_manual(pinned: Pin<&mut PinnedData>) -> &mut i32 {
    // SAFETY: movable_field는 Unpin이므로 안전
    unsafe {
        &mut Pin::get_unchecked_mut(pinned).movable_field
    }
}

/// 미래에는 이렇게 간단해질 것입니다 (아직 불가능!)
/// 
/// ```rust,ignore
/// fn project_pinned_future(pinned: Pin<&mut PinnedData>) -> &mut i32 {
///     &mut pinned.movable_field  // 컴파일러가 자동으로 처리
/// }
/// ```

/// 커스텀 스마트 포인터 예제
struct MySmartPtr<T> {
    inner: Box<T>,
}

impl<T> MySmartPtr<T> {
    fn new(value: T) -> Self {
        Self { inner: Box::new(value) }
    }
}

impl<T> std::ops::Deref for MySmartPtr<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

/// 🚧 문제: 커스텀 스마트 포인터의 필드 투영
/// 
/// 일반 참조나 Box는 .field 문법이 자동으로 작동하지만,
/// 커스텀 스마트 포인터에서는 복잡합니다.
fn use_custom_ptr() {
    let ptr = MySmartPtr::new(ComplexData {
        id: 1,
        name: "test".to_string(),
        config: Config {
            enabled: true,
            timeout: 100,
        },
    });
    
    // ✅ Deref 덕분에 필드 접근은 가능
    println!("ID: {}", ptr.id);
    
    // ❌ 하지만 필드에 대한 "스마트 포인터"를 만들 수는 없음
    // let id_ptr: MySmartPtr<u32> = ptr.project_field_id(); // 불가능!
}

/// RCU + Mutex 시나리오 시뮬레이션
/// 
/// 리눅스 커널에서 흔한 패턴:
/// - 전체 데이터는 Mutex로 보호
/// - 특정 필드는 RCU로도 읽기 가능
struct ProtectedData {
    frequently_read: i32,  // RCU로 보호됨
    rarely_written: String, // Mutex로만 보호됨
}

/// 단순화된 Mutex 래퍼
struct Mutex<T> {
    data: T,
}

impl<T> Mutex<T> {
    fn new(data: T) -> Self {
        Self { data }
    }
    
    fn lock(&mut self) -> &mut T {
        println!("[MUTEX] Lock acquired");
        &mut self.data
    }
}

/// RCU 래퍼
struct Rcu<T> {
    data: T,
}

impl<T: Copy> Rcu<T> {
    fn read(&self) -> T {
        println!("[RCU] Fast read operation");
        self.data
    }
}

/// 현재의 문제점을 보여주는 함수
fn demonstrate_mutex_rcu_problem() {
    let mut mutex = Mutex::new(ProtectedData {
        frequently_read: 42,
        rarely_written: "data".to_string(),
    });
    
    println!("\n=== 현재 Rust의 한계 ===");
    
    // ❌ 문제: Mutex 잠금 없이 RCU 필드만 읽을 수 없음
    // Rust의 타입 시스템이 이를 허용하지 않음
    {
        let data = mutex.lock();
        println!("frequently_read: {}", data.frequently_read);
        // Mutex를 잠궈야만 접근 가능 → 성능 저하!
    }
    
    println!("\n=== 미래 Rust (Field Projection 후) ===");
    println!("// let rcu_field: &Rcu<i32> = mutex.project_rcu_field();");
    println!("// let value = rcu_field.read(); // Mutex 없이 빠른 읽기!");
}

fn main() {
    println!("=== Field Projection 예제 ===\n");
    
    // 1. 기본 참조 투영
    println!("1. 기본 참조 투영");
    let data = ComplexData {
        id: 123,
        name: "Rust".to_string(),
        config: Config {
            enabled: true,
            timeout: 1000,
        },
    };
    
    let id_ref = project_to_id(&data);
    let config_ref = project_to_config(&data);
    println!("   ID: {}", id_ref);
    println!("   Config: {:?}", config_ref);
    
    // 2. 가변 참조 투영
    println!("\n2. 가변 참조 투영");
    let mut data = data;
    let name_mut = project_to_name_mut(&mut data);
    name_mut.push_str(" for Linux");
    println!("   Updated name: {}", data.name);
    
    // 3. 원시 포인터 투영 (unsafe)
    println!("\n3. 원시 포인터 투영 (unsafe)");
    let mut data = data;
    unsafe {
        let id_ptr = project_raw_pointer(&mut data as *mut ComplexData);
        println!("   ID via raw pointer: {}", *id_ptr);
    }
    
    // 4. 커스텀 스마트 포인터의 한계
    println!("\n4. 커스텀 스마트 포인터");
    use_custom_ptr();
    
    // 5. RCU + Mutex 문제 시연
    demonstrate_mutex_rcu_problem();
    
    println!("\n=== 결론 ===");
    println!("Field Projection 기능이 추가되면:");
    println!("  ✅ 커스텀 스마트 포인터에서도 필드 투영 가능");
    println!("  ✅ Pin 처리가 자동화되어 boilerplate 코드 감소");
    println!("  ✅ RCU + Mutex 같은 복잡한 패턴 안전하게 구현 가능");
    println!("  ✅ 타입 안전성 유지하면서 성능 최적화");
}
